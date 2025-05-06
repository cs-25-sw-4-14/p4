use super::CodeGenerator;
use crate::ast::{Write};
use anyhow::{Result, anyhow};
use inkwell::values::{BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub(super) fn compile_write_file(&mut self, write_node: &Write) -> Result<BasicValueEnum<'ctx>> {
        // 1. Compile value (content) - Expecting i8* (string)
        let value_val = self.compile_expression(&write_node.value)?;
        let content_ptr = match value_val {
            BasicValueEnum::PointerValue(pv) => pv,
            _ => return Err(anyhow!("Write 'value' must compile to a string pointer (i8*)"))
        };

        // 2. Compile target (filename) - Expecting i8* (string)
        let target_val = self.compile_expression(&write_node.target)?;
        let filename_ptr = match target_val {
            BasicValueEnum::PointerValue(pv) => pv,
            _ => return Err(anyhow!("Write 'target' must compile to a string pointer (i8*)"))
        };

        // 3. Get C strings for "w" mode and "%s" format
        let write_mode_ptr = self.get_global_string_ptr("w", "w_mode")?;
        let format_str_ptr = self.get_global_string_ptr("%s", "s_format")?;

        // 4. Call fopen(filename, "w") -> returns FILE* (i8*)
        let file_handle = self.builder.build_call(
            self.fopen_func,
            &[filename_ptr.into(), write_mode_ptr.into()],
            "fopen_call",
        )? 
         .try_as_basic_value().left()
         .ok_or_else(|| anyhow!("fopen call failed to return a basic value"))?
         .into_pointer_value();


        // 5. Call fprintf(file_handle, "%s", content) -> returns int
        let _fprintf_ret = self.builder.build_call(
            self.fprintf_func,
            &[file_handle.into(), format_str_ptr.into(), content_ptr.into()],
            "fprintf_call",
        )?;


        // 6. Call fclose(file_handle) -> returns int
        let _fclose_ret = self.builder.build_call(
            self.fclose_func,
            &[file_handle.into()],
            "fclose_call",
        )?;

        // Return placeholder i32(0) for void expression type
        Ok(self.context.i32_type().const_int(0, false).into())
    }
}