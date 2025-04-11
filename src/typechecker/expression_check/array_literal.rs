use crate::{ast::{ArrayLiteral, Type}, typechecker::{function_table::Ftable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;

pub fn check(array_literal: &Box<ArrayLiteral>, vtable: &VTable, ftable: &Ftable) -> Result<Type> {
    if array_literal.elements.is_empty() {
        return Err(anyhow::anyhow!("Array literal is empty"));
    }

    let first_type = expression_check::check(&array_literal.elements[0], vtable, ftable)?;

    for element in &array_literal.elements[1..] {
        let element_type = expression_check::check(element, vtable, ftable)?;
        if element_type != first_type {
            return Err(anyhow::anyhow!("Array elements have different types: {:?} and {:?}", first_type, element_type));
        }
    }
    Ok(Type::Array(Box::new(first_type), array_literal.elements.len() as i64))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{ArrayLiteral, Expression, Literal};
    use crate::typechecker::expression_check;

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

        let result = expression_check::check(&expression, &vtable, &ftable).unwrap();
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

        expression_check::check(&expression, &vtable, &ftable).unwrap_err();
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

        expression_check::check(&expression, &vtable, &ftable).unwrap_err();
    }

}
