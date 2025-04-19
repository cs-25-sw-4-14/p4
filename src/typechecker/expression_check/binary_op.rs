use crate::typechecker::expression_check;
use crate::{
    ast::{BinaryOperation, Type},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    binary_operation: &BinaryOperation,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<Type> {
    let result = semantic_check_binary_op(binary_operation, v_table, f_table, s_table)?;
    match binary_operation.operator {
        _ => Ok(result.clone()),
    }
}

pub fn semantic_check_binary_op<'a>(
    binary_operation: &'a BinaryOperation,
    v_table: &'a VTable,
    f_table: &'a FTable,
    s_table: &'a STable,
) -> Result<Type> {
    let left_type = expression_check::check(&binary_operation.left, v_table, f_table, s_table)?;
    let right_type = expression_check::check(&binary_operation.right, v_table, f_table, s_table)?;

    if left_type == right_type {
        Ok(left_type)
    } else {
        Err(anyhow::anyhow!(
            "Type mismatch: {:?} {:?} {:?}",
            left_type,
            binary_operation.operator,
            right_type
        ))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{BinaryOperation, BinaryOperator, Expression, Literal};

    #[test]
    fn check_binary_op_types_match() {
        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        let expression = Expression::BinaryOp(BinaryOperation {
            left: Box::new(Expression::Literal(Literal::Int(4))),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::Literal(Literal::Int(0))),
        });

        expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap();
    }

    #[test]
    fn check_binary_op_types_not_match() {
        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        let expression = Expression::BinaryOp(BinaryOperation {
            left: Box::new(Expression::Literal(Literal::Int(4))),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::Literal(Literal::Float(0.0))),
        });

        expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap_err();
    }
}
