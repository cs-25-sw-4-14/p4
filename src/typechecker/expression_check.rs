use crate::ast::{Expression, Type};
use super::{function_table::FTable, struct_table::STable, variable_table::VTable};
use anyhow::Result;

mod array_access;
mod array_literal;
mod binary_op;
mod function_call;
mod literal;
mod struct_field_access;
mod struct_init;
mod unary_op;
mod variable;
mod type_cast;

pub fn check(expression: &Expression, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {
    match expression {
        Expression::Literal(literal) => literal::check(literal), 
        Expression::Variable(name) => variable::check(name, v_table),
        Expression::BinaryOp(binary_operation) => binary_op::check(binary_operation, v_table, f_table, s_table),
        Expression::UnaryOp(unary_op) => unary_op::check(unary_op, v_table, f_table, s_table),
        Expression::FunctionCall(function_call) => function_call::check(function_call, v_table, f_table, s_table),
        // TODO: Marck
        Expression::StructInit(_struct_init) => todo!(),
        // TODO: Marck
        Expression::StructFieldAccess(_struct_field_access) => todo!(),
        Expression::ArrayAccess(array_access) => array_access::check(array_access, v_table, f_table, s_table),
        Expression::ArrayLiteral(array_literal) => array_literal::check(array_literal, v_table, f_table, s_table),
        Expression::TypeCast(type_cast) => type_cast::check(type_cast, v_table, f_table, s_table),  
    }
}

