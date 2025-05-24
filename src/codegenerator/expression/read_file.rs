use super::CodeGenerator;
use crate::ast::Read;
use anyhow::{Result, anyhow};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub(super) fn compile_read_file(&mut self, read_node: &Read) -> Result<BasicValueEnum<'ctx>> {
        // 1. Compile filename expression - Expecting i8* (string)
        let filename_val = self.compile_expression(&read_node.value)?;
        let filename_ptr = match filename_val {
            BasicValueEnum::PointerValue(pv) => pv,
            _ => {
                return Err(anyhow!(
                    "Read 'value' (filename) must compile to a string pointer (i8*)"
                ));
            }
        };
        let fopen_func = self
            .module
            .get_function("fopen")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;
        let fread_func = self
            .module
            .get_function("fread")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;
        let fseek_func = self
            .module
            .get_function("fseek")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;
        let ftell_func = self
            .module
            .get_function("ftell")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;
        let malloc_func = self
            .module
            .get_function("malloc")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;
        let fclose_func = self
            .module
            .get_function("fclose")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;

        // 2. Get C string "r" for read mode and necessary types/constants

        let read_mode_ptr = self.get_global_string_ptr("r", "r_mode")?;
        let i64_type = self.context.i64_type();
        let i32_type = self.context.i32_type();
        let i8_type = self.context.i8_type();

        // 3. Call fopen(filename, "r") -> returns FILE* (i8*)
        let file_handle = self
            .builder
            .build_call(
                fopen_func,
                &[filename_ptr.into(), read_mode_ptr.into()],
                "fopen_call",
            )?
            .try_as_basic_value()
            .left()
            .ok_or_else(|| anyhow!("fopen call failed to return a basic value"))?
            .into_pointer_value();

        // 4. Get file size
        let seek_end_const = i32_type.const_int(2, false);
        let seek_set_const = i32_type.const_int(0, false);
        let offset_zero_i64 = i64_type.const_int(0, false);

        self.builder.build_call(
            fseek_func,
            &[
                file_handle.into(),
                offset_zero_i64.into(),
                seek_end_const.into(),
            ],
            "fseek_end",
        )?;

        let file_size_i64 = self
            .builder
            .build_call(ftell_func, &[file_handle.into()], "ftell")?
            .try_as_basic_value()
            .left()
            .ok_or_else(|| anyhow!("ftell call failed to return a value"))?
            .into_int_value();

        self.builder.build_call(
            fseek_func,
            &[
                file_handle.into(),
                offset_zero_i64.into(),
                seek_set_const.into(),
            ],
            "fseek_set",
        )?;

        // 5. Allocate buffer: malloc(file_size + 1)
        let buffer_size_result =
            self.builder
                .build_int_add(file_size_i64, i64_type.const_int(1, false), "buffer_size");
        let buffer_size = buffer_size_result?;
        let buffer_size_arg: BasicMetadataValueEnum = buffer_size.into();
        let buffer_ptr = self
            .builder
            .build_call(malloc_func, &[buffer_size_arg], "malloc_call")?
            .try_as_basic_value()
            .left()
            .ok_or_else(|| anyhow!("malloc call failed to return a basic value"))?
            .into_pointer_value();

        // 6. Read content: fread(buffer, 1, file_size, file_handle)
        let size_one_i64 = i64_type.const_int(1, false);
        let _read_count = self
            .builder
            .build_call(
                fread_func,
                &[
                    buffer_ptr.into(),
                    size_one_i64.into(),
                    file_size_i64.into(),
                    file_handle.into(),
                ],
                "fread_call",
            )?
            .try_as_basic_value()
            .left()
            .ok_or_else(|| anyhow!("fread call failed to return a basic value"))?
            .into_int_value();

        // 7. Add null terminator
        let null_terminator_ptr = unsafe {
            self.builder
                .build_gep(i8_type, buffer_ptr, &[file_size_i64], "null_term_ptr")
        }?;
        // build_store returns Result<InstructionValue>, add ? if propagating error
        self.builder
            .build_store(null_terminator_ptr, i8_type.const_int(0, false))?;

        // 8. Close file
        let _fclose_ret =
            self.builder
                .build_call(fclose_func, &[file_handle.into()], "fclose_call")?;

        self.heap_sizes.insert(buffer_ptr, buffer_size);

        // 9. Return the pointer to the buffer (Type::String -> i8*)
        Ok(buffer_ptr.into())
    }
}
