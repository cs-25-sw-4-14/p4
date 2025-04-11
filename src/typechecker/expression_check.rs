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

pub fn check(expression: &Expression, vtable: &VTable, ftable: &Ftable) -> Result<Type> {

    match expression {
        Expression::Literal(literal) => literal::check(literal), 
        Expression::Variable(name) => variable::check(name, vtable),
        Expression::BinaryOp(binary_operation) => binary_op::check(binary_operation, vtable, ftable),
        Expression::UnaryOp(unary_op) => unary_op::check(unary_op, vtable, ftable),
        Expression::FunctionCall(function_call) => function_call::check(function_call, vtable, ftable),
        // TODO: Marck
        Expression::StructInit(struct_init) => todo!(),
        // TODO: Marck
        Expression::StructFieldAccess(struct_field_access) => todo!(),
        // TODO: Rasmus
        Expression::ArrayAccess(array_access) => array_access::check(array_access, vtable, ftable),
        Expression::ArrayLiteral(array_literal) => {
            if array_literal.elements.is_empty() {
                return Err(anyhow::anyhow!("Array literal is empty"));
            }

            let first_type = check(&array_literal.elements[0], vtable, ftable)?;
            
            for element in &array_literal.elements[1..] {
                let element_type = check(element, vtable, ftable)?;
                if element_type != first_type {
                    return Err(anyhow::anyhow!("Array elements have different types: {:?} and {:?}", first_type, element_type));
                }
            }
            Ok(Type::Array(Box::new(first_type), array_literal.elements.len() as i64))
        },
        Expression::TypeCast(type_cast) => {
            check(&type_cast.expression, vtable, ftable)?;
            Ok(type_cast.target_type.clone())
        },
    }
}

#[cfg(test)]
pub mod literal_tests {
    use super::*;
    use crate::ast::Literal;

    #[test]
    fn check_literal_should_return_type() {
        let tests = vec![
            (Expression::Literal(Literal::Char('b')), Type::Char),
            (Expression::Literal(Literal::String("test".to_string())), Type::String),
            (Expression::Literal(Literal::Bool(true)), Type::Bool),
            (Expression::Literal(Literal::Float(0.1)), Type::FP64),
            (Expression::Literal(Literal::Int(0)), Type::I64),
        ];

        let vtable = VTable::new();
        let ftable = Ftable::new();

        for (expression, expected_type) in tests {
            let result = check(&expression, &vtable, &ftable).unwrap();
            assert_eq!(result, expected_type);
        }
    }
}



#[cfg(test)]
pub mod binary_op_tests {
    use super::*;
    use crate::ast::{BinaryOperation, BinaryOperator, Literal};

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

        check(&expression, &vtable, &ftable).unwrap();
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

        check(&expression, &vtable, &ftable).unwrap_err();
    }

}

#[cfg(test)]
pub mod array_literal_tests {
    use super::*;
    use crate::ast::{ArrayLiteral, Literal};

    #[test]
    fn check_array_literal_should_return_type() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::ArrayLiteral(
            Box::new(ArrayLiteral {
                elements: vec![
                    Expression::Literal(Literal::Int(1)),
                    Expression::Literal(Literal::Int(2)),
                    Expression::Literal(Literal::Int(3)),
                ],
            })
        );

        let result = check(&expression, &vtable, &ftable).unwrap();
        assert_eq!(result, Type::Array(Box::new(Type::I64), 3));
    }

    #[test]
    fn check_array_literal_should_return_type_mixed() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::ArrayLiteral(
            Box::new(ArrayLiteral {
                elements: vec![
                    Expression::Literal(Literal::Int(1)),
                    Expression::Literal(Literal::Float(2.0)),
                    Expression::Literal(Literal::Int(3)),
                ],
            })
        );

        check(&expression, &vtable, &ftable).unwrap_err();
    }

#[test]
    fn check_array_literal_empty() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::ArrayLiteral(
            Box::new(ArrayLiteral {
                elements: vec![],
            })
        );

        check(&expression, &vtable, &ftable).unwrap_err();
    }

}

#[cfg(test)]
pub mod type_cast_tests {

    use super::*;
    use crate::ast::{Literal, TypeCast};

    #[test]
    fn check_type_cast_should_return_type() {
        let vtable = VTable::new();
        let ftable = Ftable::new();

        let expression = Expression::TypeCast(
            TypeCast {
                expression: Box::new(Expression::Literal(Literal::Int(4))),
                target_type: Type::I64,
            }
        );
        let result = check(&expression, &vtable, &ftable).unwrap();
        assert_eq!(result, Type::I64);
    }
}


