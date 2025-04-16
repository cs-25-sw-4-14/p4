use crate::ast::Expression;
use super::CodeGenerator;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

pub mod variable;
pub mod literal;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_expression(&mut self, expr: &Expression) -> Result<BasicValueEnum<'ctx>> {
        match expr {
            Expression::Literal(literal) => self.compile_literal(literal),  
            Expression::Variable(variable) => self.compile_variable(variable),
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
    }
}
