use super::{function_table::FTable, struct_table::STable, variable_table::VTable};
use crate::ast::{Expression, Type};
use anyhow::Result;

mod array_access;
mod array_literal;
mod binary_op;
mod function_call;
mod literal;
mod struct_field_access;
mod struct_init;
mod type_cast;
mod unary_op;
mod variable;
mod vector_access;
mod vector_literal;

pub fn check(
    expression: &Expression,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<Type> {
    match expression {
        Expression::Literal(literal) => literal::check(literal),
        Expression::Variable(name) => variable::check(name, v_table),
        Expression::BinaryOp(binary_operation) => {
            binary_op::check(binary_operation, v_table, f_table, s_table)
        }
        Expression::UnaryOp(unary_op) => unary_op::check(unary_op, v_table, f_table, s_table),
        Expression::FunctionCall(function_call) => {
            function_call::check(function_call, v_table, f_table, s_table)
        }
        Expression::StructInit(_struct_init) => {
            struct_init::check(_struct_init, v_table, f_table, s_table)
        }
        Expression::StructFieldAccess(_struct_field_access) => {
            struct_field_access::check(_struct_field_access, v_table, f_table, s_table)
        }
        Expression::ArrayAccess(array_access) => {
            array_access::check(array_access, v_table, f_table, s_table)
        }
        Expression::ArrayLiteral(array_literal) => {
            array_literal::check(array_literal, v_table, f_table, s_table)
        }
        Expression::VectorLiteral(vector_literal) => {
            vector_literal::check(vector_literal, v_table, f_table, s_table)
        }
        Expression::VectorAccess(vector_access) => {
            vector_access::check(vector_access, v_table, f_table, s_table)
        }
        Expression::TypeCast(type_cast) => type_cast::check(type_cast, v_table, f_table, s_table),
        Expression::Read(_) => todo!("Read expression not implemented"),
        Expression::Reference(expr, is_mutable) => {
            let inner_type = check(expr, v_table, f_table, s_table)?;
            Ok(Type::Reference(Box::new(inner_type), *is_mutable))
        }
        Expression::Dereference(expr) => {
            let expr_type = check(expr, v_table, f_table, s_table)?;
            match expr_type {
                Type::Reference(inner_type, _) => Ok(*inner_type),
                _ => Err(anyhow::anyhow!(
                    "Cannot dereference non-reference type: {:?}",
                    expr_type
                )),
            }
        }
    }
}
