use super::CodeGenerator;
use crate::ast::Read;
use anyhow::{Result, anyhow};
use inkwell::{AddressSpace, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub(super) fn compile_read_file(&mut self, read_node: &Read) -> Result<BasicValueEnum<'ctx>> {
        // 1. Compile filename
        let filename_struct = self.compile_expression(&read_node.value)?;
        let filename_val = self
            .get_string_data_ptr(filename_struct.into_pointer_value())?
            .into();

        let filename_ptr = match filename_val {
            BasicValueEnum::PointerValue(pv) => pv,
            _ => return Err(anyhow!("Read filename must compile to i8*")),
        };

        // grab libc fns:
        let fopen = self.module.get_function("fopen").unwrap();
        let fread = self.module.get_function("fread").unwrap();
        let fseek = self.module.get_function("fseek").unwrap();
        let ftell = self.module.get_function("ftell").unwrap();
        let malloc = self.module.get_function("malloc").unwrap();
        let fclose = self.module.get_function("fclose").unwrap();

        let ctx = &self.context;
        let i64_t = ctx.i64_type();
        let i32_t = ctx.i32_type();
        let i8_t = ctx.i8_type();

        // --- 2. Open file ---
        let read_mode = self.get_global_string_ptr("r", "r_mode")?;
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

        // --- 3. Determine size (ftell) ---
        let zero_i64 = i64_t.const_zero();
        let seek_end = i32_t.const_int(2, false); // SEEK_END
        let seek_set = i32_t.const_zero(); // SEEK_SET

        self.builder.build_call(
            fseek,
            &[file.into(), zero_i64.into(), seek_end.into()],
            "fseek_end",
        )?;

        let size_val = self
            .builder
            .build_call(ftell, &[file.into()], "ftell")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_int_value();

        self.builder.build_call(
            fseek,
            &[file.into(), zero_i64.into(), seek_set.into()],
            "fseek_set",
        )?;

        // --- 4. Compute total allocation size: 8 bytes for length + file size + 1 for null terminator ---
        let header_size = i64_t.const_int(8, false); // sizeof(i64) for length
        let null_term_size = i64_t.const_int(1, false); // null terminator
        let total_size = self.builder.build_int_add(
            self.builder
                .build_int_add(header_size, size_val, "size_plus_header")?,
            null_term_size,
            "total_size",
        )?;

        // --- 5. malloc that many bytes ---
        let raw_ptr = self
            .builder
            .build_call(malloc, &[total_size.into()], "malloc_read")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // --- 6. Store length at the beginning (first 8 bytes) ---
        let length_ptr = self.builder.build_pointer_cast(
            raw_ptr,
            self.context.ptr_type(AddressSpace::default()),
            "length_ptr",
        )?;
        self.builder.build_store(length_ptr, size_val)?;

        // --- 7. Calculate data pointer (skip the 8-byte length header) ---
        let data_ptr = unsafe {
            self.builder
                .build_gep(i8_t, raw_ptr, &[header_size], "data_ptr")
        }?;

        // --- 8. Read file content directly into data section ---
        _ = self.builder.build_call(
            fread,
            &[
                data_ptr.into(),
                i64_t.const_int(1, false).into(), // size of each element
                size_val.into(),                  // number of elements
                file.into(),
            ],
            "fread_call",
        )?;

        // --- 9. Null-terminate the string ---
        let null_ptr = unsafe {
            self.builder
                .build_gep(i8_t, data_ptr, &[size_val], "null_ptr")
        }?;
        self.builder.build_store(null_ptr, i8_t.const_zero())?;

        // --- 10. Close file ---
        self.builder
            .build_call(fclose, &[file.into()], "fclose_call")?;

        // --- 11. Return the raw pointer (which points to the struct) ---
        Ok(raw_ptr.into())
    }
}
