use crate::{
    ast::{ArrayAccess, Expression, Literal, Type, VariableDeclaration},
    typechecker::Typechecker,
};
use anyhow::Result;

impl<'a> Typechecker<'a> {
    pub fn check_array_access(&self, array_access: &Box<ArrayAccess>) -> Result<Type> {
        let array = self.semantic_check_array_definition(array_access)?;
        let array_type = Self::semantic_check_array_type(array)?;
        let array_length = Self::semantic_check_array_length(array)?;
        self.semantic_check_array_index(&array_access.index, array_length)?;

        Ok(array_type.clone())
    }

    pub fn semantic_check_array_definition(
        &'a self,
        array_access: &'a Box<ArrayAccess>,
    ) -> Result<&'a VariableDeclaration> {
        let array = self
            .v_table
            .get(&array_access.array_name)
            .ok_or_else(|| anyhow::anyhow!("array: {}, not defined", array_access.array_name))?;

        match &array.var_type {
            Type::Array(_, _) => Ok(array),
            _ => Err(anyhow::anyhow!(
                "Array access on non-array type: {:?}",
                array.var_type
            )),
        }
    }

    pub fn semantic_check_array_type(array: &VariableDeclaration) -> Result<&Type> {
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

    pub fn semantic_check_array_length(array: &VariableDeclaration) -> Result<&u64> {
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

    pub fn semantic_check_array_index(
        &self,
        array_index: &'a Expression,
        array_length: &'a u64,
    ) -> Result<()> {
        match self.check_expression(array_index)? {
            Type::I64 => {
                if let Expression::Literal(Literal::Int(index)) = array_index {
                    if *index >= *array_length {
                        return Err(anyhow::anyhow!(
                            "Array access index out of bounds: {}",
                            index
                        ));
                    }
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "Array access by non-integer expression: {:?}",
                    array_index
                ));
            }
        }
        Ok(())
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
