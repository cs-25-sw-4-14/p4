use crate::typechecker::expression_check;
use crate::{
    ast::{ArrayLiteral, Type},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    array_literal: &Box<ArrayLiteral>,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<Type> {
    semantic_check_literal_not_empty(array_literal)?;

    let array_type =
        semantic_check_all_elements_same_type(array_literal, v_table, f_table, s_table)?;

    Ok(Type::Array(
        Box::new(array_type.clone()),
        array_literal.elements.len() as u64,
    ))
}

pub fn semantic_check_literal_not_empty(array_literal: &Box<ArrayLiteral>) -> Result<()> {
    if array_literal.elements.is_empty() {
        return Err(anyhow::anyhow!("Array literal is empty"));
    }
    Ok(())
}

pub fn semantic_check_all_elements_same_type<'a>(
    array_literal: &'a Box<ArrayLiteral>,
    v_table: &'a VTable,
    f_table: &'a FTable,
    s_table: &'a STable,
) -> Result<Type> {
    let first_type =
        expression_check::check(&array_literal.elements[0], v_table, f_table, s_table)?;

    for element in &array_literal.elements[1..] {
        let element_type = expression_check::check(element, v_table, f_table, s_table)?;
        if element_type != first_type {
            return Err(anyhow::anyhow!(
                "Array elements have different types: {:?} and {:?}",
                first_type,
                element_type
            ));
        }
    }
    Ok(first_type)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{ArrayLiteral, Expression, Literal};
    use crate::typechecker::expression_check;

    #[test]
    fn check_array_literal_should_return_type() {
        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        let expression = Expression::ArrayLiteral(Box::new(ArrayLiteral {
            elements: vec![
                Expression::Literal(Literal::Int(1)),
                Expression::Literal(Literal::Int(2)),
                Expression::Literal(Literal::Int(3)),
            ],
        }));

        let result = expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap();
        assert_eq!(result, Type::Array(Box::new(Type::I64), 3));
    }

    #[test]
    fn check_array_literal_should_return_type_mixed() {
        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        let expression = Expression::ArrayLiteral(Box::new(ArrayLiteral {
            elements: vec![
                Expression::Literal(Literal::Int(1)),
                Expression::Literal(Literal::Float(2.0)),
                Expression::Literal(Literal::Int(3)),
            ],
        }));

        expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap_err();
    }

    #[test]
    fn check_array_literal_empty() {
        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        let expression = Expression::ArrayLiteral(Box::new(ArrayLiteral { elements: vec![] }));

        expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap_err();
    }
}
