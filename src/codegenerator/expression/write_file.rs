use super::CodeGenerator;
use crate::ast::Write;
use anyhow::{Result, anyhow};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a `write` expression that writes a string to a file.
    /// It performs: fopen(filename, "w"); fprintf(file, "%s", content); fclose(file);
    pub(super) fn compile_write_file(
        &mut self,
        write_node: &Write,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Retrieve the `fopen` function from the LLVM module.
        // It is used to open a file in write mode.
        let fopen_func = self
            .module
            .get_function("fopen")
            .ok_or_else(|| anyhow!("'fopen' function not declared in module"))?;

        // Retrieve the `fprintf` function from the module.
        // It is used to write formatted data to the file.
        let fprintf_func = self
            .module
            .get_function("fprintf")
            .ok_or_else(|| anyhow!("'fprintf' function not declared in module"))?;

        // Retrieve the `fclose` function from the module.
        // It is used to close the opened file after writing.
        let fclose_func = self
            .module
            .get_function("fclose")
            .ok_or_else(|| anyhow!("'fclose' function not declared in module"))?;

        // 1. Compile the `value` expression (content to write).
        // We expect this to evaluate to a pointer to a string (i8*).
        let value_val = self.compile_expression(&write_node.value)?;
        let content_ptr = match value_val {
            BasicValueEnum::PointerValue(pv) => pv,
            _ => {
                return Err(anyhow!(
                    "Write 'value' must compile to a string pointer (i8*)"
                ));
            }
        };

        // 2. Compile the `target` expression (filename to write to).
        // This must also evaluate to a pointer to a string (i8*).
        let target_val = self.compile_expression(&write_node.target)?;
        let filename_ptr = match target_val {
            BasicValueEnum::PointerValue(pv) => pv,
            _ => {
                return Err(anyhow!(
                    "Write 'target' must compile to a string pointer (i8*)"
                ));
            }
        };

        // 3. Prepare C-compatible global string constants:
        // - "w" for write mode
        // - "%s" as the format string for fprintf
        let write_mode_ptr = self.get_global_string_ptr("w", "w_mode")?;
        let format_str_ptr = self.get_global_string_ptr("%s", "s_format")?;

        // 4. Call fopen(filename, "w") to open the file in write mode.
        // Returns a FILE* (i8*), stored in `file_handle`.
        let file_handle = self
            .builder
            .build_call(
                fopen_func,
                &[filename_ptr.into(), write_mode_ptr.into()],
                "fopen_call",
            )?
            .try_as_basic_value()
            .left()
            .ok_or_else(|| anyhow!("fopen call failed to return a basic value"))?
            .into_pointer_value();

        // 5. Call fprintf(file_handle, "%s", content) to write the content string to the file.
        // This writes the entire content as a C string.
        let _fprintf_ret = self.builder.build_call(
            fprintf_func,
            &[
                file_handle.into(),      // FILE*
                format_str_ptr.into(),   // "%s"
                content_ptr.into(),      // content string
            ],
            "fprintf_call",
        )?;

        // 6. Call fclose(file_handle) to properly close the file after writing.
        // This flushes buffers and releases the file handle.
        let _fclose_ret =
            self.builder
                .build_call(fclose_func, &[file_handle.into()], "fclose_call")?;

        // Return a dummy `i32` value (0) as a placeholder,
        // since this expression doesn’t return a meaningful runtime value.
        Ok(self.context.i32_type().const_int(0, false).into())
    }
}
