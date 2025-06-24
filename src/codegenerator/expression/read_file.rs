use super::CodeGenerator;
use crate::ast::Read;
use anyhow::{Result, anyhow};
use inkwell::{AddressSpace, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub(super) fn compile_read_file(&mut self, read_node: &Read) -> Result<BasicValueEnum<'ctx>> {
        // 1. Compile the filename expression to get a pointer to the string struct
        let filename_struct = self.compile_expression(&read_node.value)?;

        // Extract a pointer to the actual C-style string data inside the struct
        let filename_val = self
            .get_string_data_ptr(filename_struct.into_pointer_value())?  // custom helper to get char*
            .into();

        // Ensure the filename pointer is of pointer type (i8*)
        let filename_ptr = match filename_val {
            BasicValueEnum::PointerValue(pv) => pv,
            _ => return Err(anyhow!("Read filename must compile to i8*")),
        };

        // 2. Get references to standard libc file I/O functions from LLVM module
        let fopen = self.module.get_function("fopen").unwrap();
        let fread = self.module.get_function("fread").unwrap();
        let fseek = self.module.get_function("fseek").unwrap();
        let ftell = self.module.get_function("ftell").unwrap();
        let malloc = self.module.get_function("malloc").unwrap();
        let fclose = self.module.get_function("fclose").unwrap();

        // Helper LLVM types for integers and bytes
        let ctx = &self.context;
        let i64_t = ctx.i64_type();
        let i32_t = ctx.i32_type();
        let i8_t = ctx.i8_type();

        // --- 3. Open the file in read mode ---
        // Create a global string pointer for mode "r"
        let read_mode = self.get_global_string_ptr("r", "r_mode")?;

        // Call fopen(filename, "r") to open the file
        let file = self
            .builder
            .build_call(
                fopen,
                &[filename_ptr.into(), read_mode.into()],
                "fopen_call",
            )?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // --- 4. Determine file size by seeking to end and telling position ---
        let zero_i64 = i64_t.const_zero();
        let seek_end = i32_t.const_int(2, false); // SEEK_END constant
        let seek_set = i32_t.const_zero();        // SEEK_SET constant

        // Seek to end of file: fseek(file, 0, SEEK_END)
        self.builder.build_call(
            fseek,
            &[file.into(), zero_i64.into(), seek_end.into()],
            "fseek_end",
        )?;

        // Get the file size with ftell(file)
        let size_val = self
            .builder
            .build_call(ftell, &[file.into()], "ftell")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value();

        // Seek back to start of file: fseek(file, 0, SEEK_SET)
        self.builder.build_call(
            fseek,
            &[file.into(), zero_i64.into(), seek_set.into()],
            "fseek_set",
        )?;

        // --- 5. Compute total memory size to allocate: 8 bytes for length + file size + 1 null terminator ---
        let header_size = i64_t.const_int(8, false);   // size for length field (u64)
        let null_term_size = i64_t.const_int(1, false); // null terminator size (1 byte)

        // total_size = header_size + size_val + null_term_size
        let total_size = self.builder.build_int_add(
            self.builder
                .build_int_add(header_size, size_val, "size_plus_header")?,
            null_term_size,
            "total_size",
        )?;

        // --- 6. Allocate the memory with malloc(total_size) ---
        let raw_ptr = self
            .builder
            .build_call(malloc, &[total_size.into()], "malloc_read")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // --- 7. Store the file size (length) in the first 8 bytes of allocated memory ---
        let length_ptr = self.builder.build_pointer_cast(
            raw_ptr,
            self.context.ptr_type(AddressSpace::default()),
            "length_ptr",
        )?;
        self.builder.build_store(length_ptr, size_val)?;

        // --- 8. Calculate pointer to the actual data region (skip 8-byte length header) ---
        let data_ptr = unsafe {
            self.builder
                .build_gep(i8_t, raw_ptr, &[header_size], "data_ptr")
        }?;

        // --- 9. Read file contents into data_ptr using fread ---
        // fread(data_ptr, 1, size_val, file)
        _ = self.builder.build_call(
            fread,
            &[
                data_ptr.into(),
                i64_t.const_int(1, false).into(),
                size_val.into(),
                file.into(),
            ],
            "fread_call",
        )?;

        // --- 10. Null-terminate the data buffer at data_ptr[size_val] ---
        let null_ptr = unsafe {
            self.builder
                .build_gep(i8_t, data_ptr, &[size_val], "null_ptr")
        }?;
        self.builder.build_store(null_ptr, i8_t.const_zero())?;

        // --- 11. Close the file ---
        self.builder
            .build_call(fclose, &[file.into()], "fclose_call")?;

        // --- 12. Return pointer to the allocated struct containing length + data ---
        Ok(raw_ptr.into())
    }
}
