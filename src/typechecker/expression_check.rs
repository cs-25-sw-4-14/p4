use crate::ast::{Expression, Type, UnaryOperator};
use super::{function_table::Ftable, variable_table::VTable};
use anyhow::Result;

pub fn check(expression: &Expression, vtable: &VTable, ftable: &Ftable) -> Result<Type> {

    match expression {
        Expression::Literal(literal) => Ok(Type::from(literal.clone())),
        Expression::Variable(name) => {
            if let Some(var) = vtable.get(name) {
                Ok(var.var_type.clone())
            } else {
                Err(anyhow::anyhow!("Variable {} not found in vtable", name))
            }
        }
        // TODO: Mads
        Expression::BinaryOp(binary_operation) => todo!(),
        Expression::UnaryOp(unary_op) => {
            let operand_type = check(&unary_op.operand, vtable, ftable)?;
            match unary_op.operator {
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
        // TODO: Thorbjoern
        Expression::FunctionCall(function_call) => todo!(),
        // TODO: Marck
        Expression::StructInit(struct_init) => todo!(),
        // TODO: Marck
        Expression::StructFieldAccess(struct_field_access) => todo!(),
        // TODO: Rasmus
        Expression::ArrayAccess(array_access) => todo!(),
        // TODO: Mads
        Expression::ArrayLiteral(array_literal) => todo!(),
        // TODO: Mads
        Expression::TypeCast(type_cast) => todo!(),
    }
}

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
