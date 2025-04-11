use crate::ast::{Literal, Type};
use anyhow::Result;

pub fn check(literal: &Literal) -> Result<Type> {
    Ok(Type::from(literal.clone()))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{ast::{Expression, Literal}, typechecker::{function_table::Ftable, variable_table::VTable}};
    use crate::typechecker::expression_check;

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
            let result = expression_check::check(&expression, &vtable, &ftable).unwrap();
            assert_eq!(result, expected_type);
        }
    }
}
