use crate::ast::BinaryOperator::*;
use crate::ast::{BinaryOperation, Type};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_binary_op(&mut self, binary_operation: &BinaryOperation) -> Result<Type> {
        let result = self.semantic_check_binary_op(binary_operation)?;
        Ok(result.clone())
    }

    pub fn semantic_check_binary_op(&mut self, binary_operation: &BinaryOperation) -> Result<Type> {
        let left_type = self.check_expression(&binary_operation.left)?;
        let right_type = self.check_expression(&binary_operation.right)?;

        if left_type == right_type {
            if binary_operation.operator == Equal
                || binary_operation.operator == NotEqual
                || binary_operation.operator == LessThan
                || binary_operation.operator == GreaterThan
                || binary_operation.operator == LessThanOrEqual
                || binary_operation.operator == GreaterThanOrEqual
                || binary_operation.operator == And
                || binary_operation.operator == Or
            {
                return Ok(Type::Bool);
            } else if binary_operation.operator == Multiply
                || binary_operation.operator == Divide
                || binary_operation.operator == Modulus
                || binary_operation.operator == Add
                || binary_operation.operator == Subtract
            {
                return Ok(left_type.clone());
            }
            Err(anyhow::anyhow!(
                "Invalid binary operation: {:?} {:?} {:?}",
                left_type,
                binary_operation.operator,
                right_type
            ))
        } else {
            Err(anyhow::anyhow!(
                "Type mismatch: {:?} {:?} {:?}",
                left_type,
                binary_operation.operator,
                right_type
            ))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        ast::{BinaryOperation, BinaryOperator, Expression, Literal},
        debugger::Debugger,
    };

    #[test]
    fn check_binary_op_types_match() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        let expression = Expression::BinaryOp(BinaryOperation {
            left: Box::new(Expression::Literal(Literal::Int(4))),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::Literal(Literal::Int(0))),
        });

        typechecker.check_expression(&expression).unwrap();
    }

    #[test]
    fn check_binary_op_types_not_match() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        let expression = Expression::BinaryOp(BinaryOperation {
            left: Box::new(Expression::Literal(Literal::Int(4))),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::Literal(Literal::Float(0.0))),
        });

        typechecker.check_expression(&expression).unwrap_err();
    }
}
