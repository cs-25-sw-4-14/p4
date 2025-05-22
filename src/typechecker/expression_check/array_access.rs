use crate::{
    ast::{ArrayAccess, Expression, Literal, Type},
    typechecker::Typechecker,
};
use anyhow::{Result, bail};

impl Typechecker<'_> {
    pub fn check_array_access(&mut self, array_access: &ArrayAccess) -> Result<Type> {
        // Scope to end immutable borrow before calling `self.check_expression`
        let (array_type, array_length) = {
            let array = self.v_table.get(&array_access.array_name).ok_or_else(|| {
                anyhow::anyhow!("array: {}, not defined", array_access.array_name)
            })?;

            match &array.variable_declaration.var_type {
                Type::Array(inner_type, length) => (inner_type.as_ref().clone(), *length),
                other => {
                    bail!("Array access on non-array type: {:?}", other);
                }
            }
        };

        // Immutable borrow has ended — now we can call `self.check_expression`
        match self.check_expression(&array_access.index)? {
            Type::I64 => {
                if let Expression::Literal(Literal::Int(index)) = array_access.index {
                    if index >= array_length {
                        return Err(anyhow::anyhow!(
                            "Array access index out of bounds: {}",
                            index
                        ));
                    }
                }
            }
            other => {
                return Err(anyhow::anyhow!(
                    "Array access by non-integer expression: {:?}",
                    other
                ));
            }
        }

        Ok(array_type)
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
        let mut typechecker = Typechecker::new(&debugger);

        for (expression, expected_type) in tests {
            let result = typechecker.check_expression(&expression).unwrap();
            assert_eq!(result, expected_type);
        }
    }
}
