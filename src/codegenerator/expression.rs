use crate::ast::Expression;
use super::CodeGenerator;
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
// Compile an expression.
    pub fn compile_expression(&mut self, expr: &Expression) -> Result<()> {
        match expr {
            // EXAMPLE
            // Expression::Literal(literal) => self.compile_literal(literal),
            Expression::Literal(literal) => todo!(),
            Expression::Variable(variable) => todo!(),
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
