use super::Typechecker;
use crate::ast::{Expression, Type};
use anyhow::Result;

mod array_access;
mod array_literal;
mod binary_op;
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
    pub fn check_expression(&mut self, expression: &Expression) -> Result<Type> {
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
            Expression::Reference(name, is_mutable) => {
                if *is_mutable {
                    self.v_table.borrow_mut(name)?;
                } else {
                    self.v_table.borrow_immut(name)?;
                }

                let var_type = self
                    .v_table
                    .get(name)
                    .ok_or(anyhow::anyhow!(
                        "Variable '{}' not found when attempting to borrow",
                        name
                    ))?
                    .variable_declaration
                    .var_type
                    .clone();

                Ok(Type::Reference(Box::new(var_type), *is_mutable))
            }
            Expression::Dereference(name) => {
                let var_type = self
                    .v_table
                    .get(name)
                    .ok_or(anyhow::anyhow!("Variable '{}' not found", name))?
                    .variable_declaration
                    .var_type
                    .clone();

                if let Type::Reference(var_type, _) = var_type {
                    Ok(*var_type)
                } else {
                    return Err(anyhow::anyhow!(
                        "Cannot dereference variable '{}' of type '{:?}'",
                        name,
                        var_type
                    ));
                }
            }
            Expression::Read(read_expr) => self.check_read_file(read_expr),
            Expression::Write(write_expr) => self.check_write_file(write_expr),
            Expression::Null => Ok(Type::Null),
        }
    }
}
