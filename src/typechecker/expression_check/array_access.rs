use crate::typechecker::expression_check;
use crate::{
    ast::{ArrayAccess, Expression, Literal, Type, VariableDeclaration},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    array_access: &Box<ArrayAccess>,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<Type> {
    let array = semantic_check_array_definition(array_access, v_table)?;
    let array_type = semantic_check_array_type(array)?;
    let array_length = semantic_check_array_length(array)?;
    semantic_check_array_index(&array_access.index, array_length, v_table, f_table, s_table)?;

    Ok(array_type.clone())
}

pub fn semantic_check_array_definition<'a>(
    array_acces: &'a Box<ArrayAccess>,
    v_table: &'a VTable,
) -> Result<&'a VariableDeclaration> {
    let array = v_table
        .get(&array_acces.array_name)
        .ok_or_else(|| anyhow::anyhow!("array: {}, not defined", array_acces.array_name))?;

    match &array.var_type {
        Type::Array(_, _) => Ok(array),
        _ => Err(anyhow::anyhow!(
            "Array access on non-array type: {:?}",
            array.var_type
        )),
    }
}

pub fn semantic_check_array_type<'a>(array: &'a VariableDeclaration) -> Result<&'a Type> {
    let array_type = match &array.var_type {
        Type::Array(ty, _) => ty,
        _ => {
            return Err(anyhow::anyhow!(
                "Array access on non-array type: {:?}",
                array.var_type
            ));
        }
    };
    Ok(array_type)
}

pub fn semantic_check_array_length<'a>(array: &'a VariableDeclaration) -> Result<&'a i64> {
    let array_length = match &array.var_type {
        Type::Array(_, length) => length,
        _ => {
            return Err(anyhow::anyhow!(
                "Array access on non-array type: {:?}",
                array.var_type
            ));
        }
    };
    Ok(array_length)
}

pub fn semantic_check_array_index<'a>(
    array_index: &'a Expression,
    array_length: &'a i64,
    v_table: &'a VTable,
    f_table: &'a FTable,
    s_table: &'a STable,
) -> Result<()> {
    match expression_check::check(array_index, v_table, f_table, s_table)? {
        Type::I64 => match array_index {
            Expression::Literal(Literal::Int(index)) => {
                if *index < 0 || *index >= *array_length {
                    return Err(anyhow::anyhow!(
                        "Array access index out of bounds: {}",
                        index
                    ));
                }
            }
            _ => {}
        },
        _ => {
            return Err(anyhow::anyhow!(
                "Array access by non-integer expression: {:?}",
                array_index
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::typechecker::expression_check;
    use crate::{
        ast::{Expression, Literal},
        typechecker::{function_table::FTable, variable_table::VTable},
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
