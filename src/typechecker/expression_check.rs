use crate::ast::{Expression, Type};
use super::{function_table::Ftable, variable_table::VTable};
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

pub fn check(expression: &Expression, vtable: &VTable, ftable: &Ftable) -> Result<Type> {
    match expression {
        Expression::Literal(literal) => literal::check(literal), 
        Expression::Variable(name) => variable::check(name, vtable),
        Expression::BinaryOp(binary_operation) => binary_op::check(binary_operation, vtable, ftable),
        Expression::UnaryOp(unary_op) => unary_op::check(unary_op, vtable, ftable),
        Expression::FunctionCall(function_call) => function_call::check(function_call, vtable, ftable),
        // TODO: Marck
        Expression::StructInit(_struct_init) => todo!(),
        // TODO: Marck
        Expression::StructFieldAccess(_struct_field_access) => todo!(),
        Expression::ArrayAccess(array_access) => array_access::check(array_access, vtable, ftable),
        Expression::ArrayLiteral(array_literal) => array_literal::check(array_literal, vtable, ftable),
        Expression::TypeCast(type_cast) => type_cast::check(type_cast, vtable, ftable),  
    }
}

