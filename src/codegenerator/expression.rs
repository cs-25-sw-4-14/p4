use super::CodeGenerator;
use crate::ast::Expression;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

pub mod binary_op;
pub mod function_call;
pub mod literal;
pub mod unary_op;
pub mod variable;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_expression(&mut self, expr: &Expression) -> Result<BasicValueEnum<'ctx>> {
        match expr {
            Expression::Literal(literal) => self.compile_literal(literal),
            Expression::Variable(variable) => self.compile_variable(variable),
            Expression::BinaryOp(binary_operation) => self.compile_binary_op(binary_operation),
            Expression::UnaryOp(unary_operation) => self.compile_unary_op(unary_operation),
            Expression::FunctionCall(function_call) => self.compile_function_call(function_call),
            Expression::StructInit(struct_init) => todo!(),
            Expression::StructFieldAccess(struct_field_access) => todo!(),
            Expression::ArrayAccess(array_access) => todo!(),
            Expression::ArrayLiteral(array_literal) => todo!(),
            Expression::TypeCast(type_cast) => todo!(),
        }
    }
}
