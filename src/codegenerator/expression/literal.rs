use super::CodeGenerator;
use crate::ast::Literal;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_literal(&mut self, literal: &Literal) -> Result<BasicValueEnum<'ctx>> {
        Ok(match literal {
            Literal::Int(value) => {
                let u64_type = self.context.i64_type();
                let u64_value = u64_type.const_int(*value, false);
                BasicValueEnum::IntValue(u64_value)
            }
            Literal::Float(value) => {
                let f64_type = self.context.f64_type();
                let f64_value = f64_type.const_float(*value);
                BasicValueEnum::FloatValue(f64_value)
            }
            Literal::Bool(value) => {
                let bool_type = self.context.bool_type();
                let bool_value = bool_type.const_int(if *value { 1 } else { 0 }, false);
                BasicValueEnum::IntValue(bool_value)
            }
            Literal::String(value) => {
                let string_value = self
                    .builder
                    .build_global_string_ptr(value, "string_literal")?;
                let ptr = string_value.as_pointer_value();
                BasicValueEnum::PointerValue(ptr)
            }
            Literal::Char(value) => {
                let char_type = self.context.i32_type();
                let char_value = char_type.const_int(*value as u64, false);
                BasicValueEnum::IntValue(char_value)
            }
        })
    }
}
