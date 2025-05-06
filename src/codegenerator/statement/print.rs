use crate::{ast::Debug, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::values::BasicValueEnum;

impl CodeGenerator<'_> {
    pub fn compile_print_stmt(&mut self, stmt: &Debug) -> Result<()> {
        let value = self.compile_expression(&stmt.expression)?;

        let printf = self
            .module
            .get_function("printf")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;

        let call_args = match value {
            BasicValueEnum::IntValue(int_val) => match int_val.get_type().get_bit_width() {
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
            },
            BasicValueEnum::FloatValue(float_val) => {
                match self.get_float_bit_width(float_val.get_type()) {
                    Ok(16) => {
                        let fmt_str = self
                            .builder
                            .build_global_string_ptr("16: %0.40f\n", "fmt_f16")?;
                        let extended = self.builder.build_float_ext(
                            float_val,
                            self.context.f32_type(),
                            "f16_to_f32",
                        );
                        vec![fmt_str.as_pointer_value().into(), extended.unwrap().into()]
                    }
                    Ok(32) => {
                        let fmt_str = self
                            .builder
                            .build_global_string_ptr("32: %0.40f\n", "fmt_f32")?;
                        vec![fmt_str.as_pointer_value().into(), float_val.into()]
                    }
                    Ok(64) => {
                        let fmt_str = self
                            .builder
                            .build_global_string_ptr("64: %.040lf\n", "fmt_f64")?;
                        vec![fmt_str.as_pointer_value().into(), float_val.into()]
                    }
                    Err(e) => {
                        return Err(anyhow!("Failed to get float bit width: {}", e));
                    }
                    _ => {
                        return Err(anyhow!("Unsupported float type"));
                    }
                }
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
