use crate::typechecker::expression_check;
use crate::{
    ast::{Type, VectorLiteral},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    vector_literal: &Box<VectorLiteral>,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<Type> {
    // Check if the vector literal is empty, then type

    //check if elements are same type
    let vector_type =
        semantic_check_all_elements_same_type(vector_literal, v_table, f_table, s_table)?;

    Ok(vector_type)
}

//test if all element are same type
pub fn semantic_check_all_elements_same_type<'a>(
    vector_literal: &'a Box<VectorLiteral>,
    v_table: &'a VTable,
    f_table: &'a FTable,
    s_table: &'a STable,
) -> Result<Type> {
    let first_type =
        expression_check::check(&vector_literal.elements[0], v_table, f_table, s_table)?;

    for element in &vector_literal.elements[1..] {
        let element_type = expression_check::check(element, v_table, f_table, s_table)?;
        if element_type != first_type {
            return Err(anyhow::anyhow!(
                "Vector elements have different types: {:?} and {:?}",
                first_type,
                element_type
            ));
        }
    }
    Ok(Type::Vector(
        Box::new(first_type),
        vector_literal.elements.len() as u64,
    ))
}

#[cfg(test)]
pub mod vector_literal_tests {
    use super::*;
    use crate::ast::{Expression, Literal};
    use crate::typechecker::expression_check;

    #[test]
    fn test_vector_literal() {
        let expression = Expression::VectorLiteral(Box::new(VectorLiteral {
            elements: vec![
                Expression::Literal(Literal::Int(1)),
                Expression::Literal(Literal::Int(2)),
                Expression::Literal(Literal::Int(3)),
            ],
        }));

        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        let result = expression_check::check(&expression, &v_table, &f_table, &s_table);
        assert!(result.is_ok());
    }
}
