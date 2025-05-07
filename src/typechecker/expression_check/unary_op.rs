use crate::ast::{Type, UnaryOperation, UnaryOperator};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl<'a> Typechecker<'a> {
    pub fn check_unary_op(&self, unary_operation: &UnaryOperation) -> Result<Type> {
        let operand_type = self.check_expression(&unary_operation.operand)?;
        match unary_operation.operator {
            UnaryOperator::Negate => match operand_type {
                Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::F16
                | Type::F32
                | Type::F64 => Ok(operand_type),
                _ => Err(anyhow::anyhow!(
                    "Unary '-' not supported for type {:?}",
                    operand_type
                )),
            },
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
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{Expression, Literal, Type, UnaryOperation};
    use crate::debugger::Debugger;

    #[test]
    fn check_unary_op_negate_ok() {
        let expression = Expression::UnaryOp(UnaryOperation {
            operator: UnaryOperator::Negate,
            operand: Box::new(Expression::Literal(Literal::Int(5))),
        });

        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);

        let result = typechecker.check_expression(&expression).unwrap();
        assert_eq!(result, Type::I64);
    }

    #[test]
    fn check_unary_op_negate_err() {
        let expression = Expression::UnaryOp(UnaryOperation {
            operator: UnaryOperator::Negate,
            operand: Box::new(Expression::Literal(Literal::Bool(false))),
        });

        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);

        typechecker.check_expression(&expression).unwrap_err();
    }

    #[test]
    fn check_unary_op_not_ok() {
        let expression = Expression::UnaryOp(UnaryOperation {
            operator: UnaryOperator::Not,
            operand: Box::new(Expression::Literal(Literal::Bool(false))),
        });

        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);
        let result = typechecker.check_expression(&expression).unwrap();
        assert_eq!(result, Type::Bool);
    }

    #[test]
    fn check_unary_op_not_err() {
        let expression = Expression::UnaryOp(UnaryOperation {
            operator: UnaryOperator::Not,
            operand: Box::new(Expression::Literal(Literal::Int(30))),
        });

        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);

        typechecker.check_expression(&expression).unwrap_err();
    }
}
