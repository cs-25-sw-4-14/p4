use crate::{ast::Print, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_print_stmt(&mut self, stmt: &Print) -> Result<()> {
        let value = self.compile_expression(&stmt.expression)?;

        let printf = self
            .module
            .get_function("printf")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;

        let call_args = match value {
            BasicValueEnum::IntValue(int_val) => {
                match int_val.get_type().get_bit_width() {
                    8 => {
                        let fmt_str = self.builder.build_global_string_ptr("%hhi\n", "fmt_i8")?;
                        vec![fmt_str.as_pointer_value().into(), int_val.into()]
                    }
                    16 => {
                        let fmt_str = self.builder.build_global_string_ptr("%hi\n", "fmt_i16")?;
                        vec![fmt_str.as_pointer_value().into(), int_val.into()]
                    }
                    32 => {
                        let fmt_str = self.builder.build_global_string_ptr("%d\n", "fmt_i32")?;
                        vec![fmt_str.as_pointer_value().into(), int_val.into()]
                    }
                    64 => {
                        let fmt_str = self.builder.build_global_string_ptr("%lld\n", "fmt_i64")?;
                        vec![fmt_str.as_pointer_value().into(), int_val.into()]
                    }
                    other => {
                        return Err(anyhow!("Unsupported integer bit width: {}", other));
                    }
                }
            }
            BasicValueEnum::FloatValue(float_val) => {
                let fmt_str = self.builder.build_global_string_ptr("%llf\n", "fmt")?;
                vec![fmt_str.as_pointer_value().into(), float_val.into()]
            }
            BasicValueEnum::PointerValue(ptr_val) => {
                let fmt_str = self.builder.build_global_string_ptr("%s\n", "fmt")?;
                vec![fmt_str.as_pointer_value().into(), ptr_val.into()]
            }
            _ => return Err(anyhow!("not a valid type for print statement")),
        };

        let _ = self.builder.build_call(printf, &call_args, "printf_call");

        Ok(())
    }
}
