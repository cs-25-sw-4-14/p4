use crate::ast::{Literal, Type};
use anyhow::Result;

pub fn check(literal: &Literal) -> Result<Type> {
    Ok(Type::from(literal.clone()))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::typechecker::expression_check;
    use crate::{
        ast::{Expression, Literal},
        typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
    };

    #[test]
    fn check_literal_should_return_type() {
        let tests = vec![
            (Expression::Literal(Literal::Char('b')), Type::Char),
            (
                Expression::Literal(Literal::String("test".to_string())),
                Type::String,
            ),
            (Expression::Literal(Literal::Bool(true)), Type::Bool),
            (Expression::Literal(Literal::Float(0.1)), Type::FP64),
            (Expression::Literal(Literal::Int(0)), Type::I64),
        ];

        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        for (expression, expected_type) in tests {
            let result =
                expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap();
            assert_eq!(result, expected_type);
        }
    }
}
