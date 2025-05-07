use crate::{
    ast::{Literal, Type},
    typechecker::Typechecker,
};
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_literal(literal: &Literal) -> Result<Type> {
        Ok(Type::from(literal.clone()))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{Expression, Literal};
    use crate::debugger::Debugger;

    #[test]
    fn check_literal_should_return_type() {
        let tests = vec![
            (Expression::Literal(Literal::Char('b')), Type::Char),
            (
                Expression::Literal(Literal::String("test".to_string())),
                Type::String,
            ),
            (Expression::Literal(Literal::Bool(true)), Type::Bool),
            (Expression::Literal(Literal::Float(0.1)), Type::F64),
            (Expression::Literal(Literal::Int(0)), Type::I64),
        ];

        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);

        for (expression, expected_type) in tests {
            let result = typechecker.check_expression(&expression).unwrap();
            assert_eq!(result, expected_type);
        }
    }
}
