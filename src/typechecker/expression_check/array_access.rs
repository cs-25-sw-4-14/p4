use crate::{
    ast::{ArrayAccess, Expression, Literal, Type},
    typechecker::Typechecker,
};
use anyhow::{Result, bail};

impl Typechecker<'_> {
    /// Performs typechecking on an array access expression.
    /// Ensures the array exists, the index is of integer type, and (if index is a literal) it's within bounds.
    pub fn check_array_access(&mut self, array_access: &ArrayAccess) -> Result<Type> {
        // Scope to end immutable borrow before calling self.check_expression later.
        // This avoids conflicting borrows on self.v_table.
        let (array_type, array_length) = {
            // Look up the array variable by name in the variable table (v_table)
            let array = self.v_table.get(&array_access.array_name).ok_or_else(|| {
                anyhow::anyhow!("array: {}, not defined", array_access.array_name)
            })?;

            // Ensure the variable is of array type
            match &array.variable_declaration.var_type {
                Type::Array(inner_type, length) => (
                    inner_type.as_ref().clone(), // Get the type of the array elements
                    *length                      // Get the declared length of the array
                ),
                other => {
                    // If not an array, return a type error
                    bail!("Array access on non-array type: {:?}", other);
                }
            }
        };

        // Now that the borrow of self.v_table has ended, we can call methods on `self` again

        // Check the type of the index expression
        match self.check_expression(&array_access.index)? {
            Type::I64 => {
                // If the index is a literal integer, check that it is within bounds
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
                // If the index is not an integer, it's a type error
                return Err(anyhow::anyhow!(
                    "Array access by non-integer expression: {:?}",
                    other
                ));
            }
        }

        // If all checks pass, return the element type of the array
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
