use crate::{ast::{BinaryOperation, Type}, typechecker::{function_table::Ftable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;

pub fn check(binary_operation: &BinaryOperation, vtable: &VTable, ftable: &Ftable) -> Result<Type> {
    let left_type = expression_check::check(&binary_operation.left, vtable, ftable)?;
    let right_type = expression_check::check(&binary_operation.right, vtable, ftable)?;

    if left_type == right_type {
        Ok(left_type)
    } else {
        Err(anyhow::anyhow!("Type mismatch: {:?} {:?} {:?}", left_type, binary_operation.operator, right_type))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{BinaryOperation, BinaryOperator, Expression, Literal};

    #[test]
    fn check_binary_op_types_match() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::BinaryOp(
            BinaryOperation {
                left: Box::new(Expression::Literal(Literal::Int(4))),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Literal(Literal::Int(0))),
            }
        );

        expression_check::check(&expression, &vtable, &ftable).unwrap();
    }

    #[test]
    fn check_binary_op_types_not_match() {
        let vtable = VTable::new();
        let ftable = Ftable::new();


        let expression = Expression::BinaryOp(
            BinaryOperation {
                left: Box::new(Expression::Literal(Literal::Int(4))),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Literal(Literal::Float(0.0))),
            }
        );

        expression_check::check(&expression, &vtable, &ftable).unwrap_err();
    }

}



