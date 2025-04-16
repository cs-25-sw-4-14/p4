use crate::ast::Expression;
use super::CodeGenerator;
use anyhow::Result;
use crate::ast::Literal;
use inkwell::values::{BasicValueEnum, PointerValue};

impl<'ctx> CodeGenerator<'ctx> {
// Compile an expression.
    pub fn compile_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            // EXAMPLE
            // Expression::Literal(literal) => self.compile_literal(literal),
            // TODO: Rasmus
            Expression::Literal(literal) => {
                match literal {
                    Literal::Int(value) => {
                        let u64_type = self.context.u64_type();
                        let u64_value = u64_type.const_int(*value, false);
                        self.last_value = BasicValueEnum::IntValue(u64_value);
                    },
                    Literal::Float(value) => {
                        let f64_type = self.context.f64_type();
                        let f64_value = f64_type.const_float(*value);
                        self.last_value = BasicValueEnum::FloatValue(f64_value);
                    },
                    Literal::Bool(value) => {
                        let bool_type = self.context.bool_type();
                        let bool_value = bool_type.const_int(if *value { 1 } else { 0 }, false);
                        self.last_value = BasicValueEnum::IntValue(bool_value);
                    },
                    Literal::String(value) => {
                        let string_value = self.builder.build_global_string_ptr(value, "string_literal");
                        self.last_value = BasicValueEnum::PointerValue(string_value.as_pointer_value());
                    },
                    Literal::Char(value) => {
                        let char_type = self.context.i32_type();
                        let char_value = char_type.const_int(*value as u64, false);
                        self.last_value = BasicValueEnum::IntValue(char_value);
                    },
                }
            },  
            // TODO: Mads
            Expression::Variable(variable) => todo!(),
            // TODO: Marc
            Expression::BinaryOp(binary_operation) => todo!(),
            Expression::UnaryOp(unary_operation) => todo!(),
            Expression::FunctionCall(function_call) => todo!(),
            Expression::StructInit(struct_init) => todo!(),
            Expression::StructFieldAccess(struct_field_access) => todo!(),
            Expression::ArrayAccess(array_access) => todo!(),
            Expression::ArrayLiteral(array_literal) => todo!(),
            Expression::TypeCast(type_cast) => todo!(),
        }
        Ok(())
    }
}
