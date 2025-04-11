
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{Expression, Literal, Type, UnaryOperation};

    #[test]
    fn check_unary_op_negate_ok() {
        let expression = Expression::UnaryOp(
            UnaryOperation {
                operator: UnaryOperator::Negate,
                operand: Box::new(Expression::Literal(Literal::Int(5))),
            }
        );

        let vtable = VTable::new();
        let ftable = Ftable::new();

        let result = check(&expression, &vtable, &ftable).unwrap();
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
        let ftable = Ftable::new();

        check(&expression, &vtable, &ftable).unwrap_err();
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
        let ftable = Ftable::new();

        let result = check(&expression, &vtable, &ftable).unwrap();
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
        let ftable = Ftable::new();

        check(&expression, &vtable, &ftable).unwrap_err();
    }
    
}
