use crate::ast::{Expression, Literal, Type};
use super::{ftable::Ftable, vtable::VTable};
use anyhow::Result;


pub struct ECheck {
}

impl ECheck {
    pub fn new() -> Self {
        ECheck {}
    }

    pub fn check_e(expression: &Expression, vtable: &VTable, ftable: &Ftable) -> Result<Type> {

        match expression {
            Expression::Literal(literal) => Ok(Type::from(literal.clone())),
            Expression::Variable(name) => {
                if let Some(var) = vtable.get(name) {
                    Ok(var.var_type.clone())
                } else {
                    Err(anyhow::anyhow!("Variable {} not found in vtable", name))
                }
            }
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
