
use crate::{ast::{ArrayAccess, Expression, Literal, Type}, typechecker::{function_table::Ftable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;

pub fn check(array_access: &Box<ArrayAccess>, vtable: &VTable, ftable: &Ftable) -> Result<Type> {

    let array_type = vtable.get(&array_access.array_name)
        .ok_or_else(|| anyhow::anyhow!("array: {}, not defined", array_access.array_name))?;

    let (element_type, length) = match &array_type.var_type {
        Type::Array(ty, len) => (ty, len),
        _ => return Err(anyhow::anyhow!(
            "Array access on non-array type: {:?}",
            array_type
        )),
    };

    match expression_check::check(&array_access.index, vtable, ftable)? {
        Type::I64 => {
            match array_access.index {
                Expression::Literal(Literal::Int(index)) => {
                    if index > *length || index < 0 {
                        return Err(anyhow::anyhow!("Array access index out of bounds: {}", index));
                    }
                }
                _ => {}
            }
        }
        _ => {
            return Err(anyhow::anyhow!("Array access by non-integer expression: {:?}", array_access.index));
        }
    }

    Ok(*element_type.clone())
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
