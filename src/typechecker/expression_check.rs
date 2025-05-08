use super::Typechecker;
use crate::ast::{Expression, Type};
use anyhow::Result;

mod array_access;
mod array_literal;
mod binary_op;
mod enum_literal;
mod function_call;
mod literal;
mod read_file;
mod struct_field_access;
mod struct_init;
mod type_cast;
mod unary_op;
mod variable;
mod vector_access;
mod vector_literal;
mod write_file;

impl Typechecker<'_> {
    pub fn check_expression(&self, expression: &Expression) -> Result<Type> {
        match expression {
            Expression::Literal(literal) => Self::check_literal(literal),
            Expression::Variable(name) => self.check_variable(name),
            Expression::BinaryOp(binary_operation) => self.check_binary_op(binary_operation),
            Expression::UnaryOp(unary_op) => self.check_unary_op(unary_op),
            Expression::FunctionCall(function_call) => self.check_function_call(function_call),
            Expression::StructInit(struct_init) => self.check_struct_init(struct_init),
            Expression::StructFieldAccess(struct_field_access) => {
                self.check_struct_field_access(struct_field_access)
            }
            Expression::ArrayAccess(array_access) => self.check_array_access(array_access),
            Expression::ArrayLiteral(array_literal) => self.check_array_literal(array_literal),
            Expression::VectorLiteral(vector_literal) => self.check_vector_literal(vector_literal),
            Expression::VectorAccess(vector_access) => self.check_vector_access(vector_access),
            Expression::TypeCast(type_cast) => self.check_type_cast(type_cast),
            // TODO: Move into seperate module
            Expression::Reference(expr, is_mutable) => {
                let inner_type = self.check_expression(expr)?;
                Ok(Type::Reference(Box::new(inner_type), *is_mutable))
            }
            // TODO: Move into seperate module
            Expression::Dereference(expr) => {
                let expr_type = self.check_expression(expr)?;
                match expr_type {
                    Type::Reference(inner_type, _) => Ok(*inner_type),
                    _ => Err(anyhow::anyhow!(
                        "Cannot dereference non-reference type: {:?}",
                        expr_type
                    )),
                }
            }
            Expression::Read(read_expr) => self.check_read_file(read_expr),
            Expression::Write(write_expr) => self.check_write_file(write_expr),
            // TODO: Handle enum literal
            Expression::EnumLiteral(_) => todo!(),
        }
    }
}
