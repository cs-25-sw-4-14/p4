use crate::{ast::{Type, UnaryOperation, UnaryOperator}, typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;

pub fn check(unary_operation: &UnaryOperation, vtable: &VTable, ftable: &FTable, stable: &STable) -> Result<Type> {
    let operand_type = expression_check::check(&unary_operation.operand, vtable, ftable, stable)?;
            match unary_operation.operator {
                UnaryOperator::Negate => {
                    match operand_type {
                        Type::I8 | Type::I16 | Type::I32 | Type::I64 |
                        Type::FP8 | Type::FP16 | Type::FP32 | Type::FP64 => Ok(operand_type),
                        _ => Err(anyhow::anyhow!(
                            "Unary '-' not supported for type {:?}",
                            operand_type
                        )),
                    }
                }
                UnaryOperator::Not => {
                    if operand_type == Type::Bool {
                        Ok(Type::Bool)
                    } else {
                        Err(anyhow::anyhow!(
                            "Unary '!' expects a Bool, but found {:?}",
                            operand_type
                        ))
                    }
                }
            }
}



#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{Expression, Literal, Type, UnaryOperation};
use crate::typechecker::expression_check;
use crate::typechecker::struct_table::STable;

    #[test]
    fn check_unary_op_negate_ok() {
        let expression = Expression::UnaryOp(
            UnaryOperation {
                operator: UnaryOperator::Negate,
                operand: Box::new(Expression::Literal(Literal::Int(5))),
            }
        );

        let vtable = VTable::new();
        let ftable = FTable::new();
        let stable = STable::new();

        let result = expression_check::check(&expression, &vtable, &ftable, &stable).unwrap();
        assert_eq!(result, Type::I64);
    }

    #[test]
    fn check_unary_op_negate_err() {
        let expression = Expression::UnaryOp(
            UnaryOperation {
                operator: UnaryOperator::Negate,
                operand: Box::new(Expression::Literal(Literal::Bool(false))),
            }
        );

        let vtable = VTable::new();
        let ftable = FTable::new();
        let stable = STable::new();

        expression_check::check(&expression, &vtable, &ftable, &stable).unwrap_err();
    }

    #[test]
    fn check_unary_op_not_ok() {
        let expression = Expression::UnaryOp(
            UnaryOperation {
                operator: UnaryOperator::Not,
                operand: Box::new(Expression::Literal(Literal::Bool(false))),
            }
        );

        let vtable = VTable::new();
        let ftable = FTable::new();
        let stable = STable::new();

        let result = expression_check::check(&expression, &vtable, &ftable, &stable).unwrap();
        assert_eq!(result, Type::Bool);
    }

    #[test]
    fn check_unary_op_not_err() {
        let expression = Expression::UnaryOp(
            UnaryOperation {
                operator: UnaryOperator::Not,
                operand: Box::new(Expression::Literal(Literal::Int(30))),
            }
        );

        let vtable = VTable::new();
        let ftable = FTable::new();
        let stable = STable::new();

        expression_check::check(&expression, &vtable, &ftable, &stable).unwrap_err();
    }

}
