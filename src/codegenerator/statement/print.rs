use crate::{ast::Print, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_print_stmt(&mut self, stmt: &Print) -> Result<()> {
        let value = self.compile_expression(&stmt.value)?;
        
        let printf = self.module.get_function("printf")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;
        
        let call_args = match value {
            BasicValueEnum::IntValue(int_val) => {
                let fmt_str = self.builder.build_global_string_ptr("%ld\n", "fmt")?;
                vec![
                    fmt_str.as_pointer_value().into(),
                    int_val.into(),
                ]
            }
            BasicValueEnum::PointerValue(ptr_val) => {
                let fmt_str = self.builder.build_global_string_ptr("%s\n", "fmt")?;
                vec![
                    fmt_str.as_pointer_value().into(),
                    ptr_val.into(),
                ]
            }
            _ => return Err(anyhow!("print() only supports integer and string types")),
        };

        let _ = self.builder.build_call(printf, &call_args, "printf_call");

        Ok(())
    }
}
