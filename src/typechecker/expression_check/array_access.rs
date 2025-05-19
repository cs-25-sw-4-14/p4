use crate::{
    ast::{ArrayAccess, BaseType, Type},
    typechecker::Typechecker,
};
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_array_access(&self, array_access: &ArrayAccess) -> Result<Type> {
        let access_type = self.check_expression(&array_access.array_expr)?;
        let array_index = self.check_expression(&array_access.index)?;

        if array_index.base != BaseType::I64 {
            return Err(anyhow::anyhow!("Array index must be of type I64"));
        }

        if let BaseType::Array(_, _) = access_type.base {
            return Ok(access_type.clone());
        } else {
            return Err(anyhow::anyhow!(
                "Array access on non-array type: {:?}",
                access_type.base
            ));
        }
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
            (
                Expression::Literal(Literal::Char('b')),
                Type::owned(BaseType::Char),
            ),
            (
                Expression::Literal(Literal::String("test".to_string())),
                Type::owned(BaseType::String),
            ),
            (
                Expression::Literal(Literal::Bool(true)),
                Type::owned(BaseType::Bool),
            ),
            (
                Expression::Literal(Literal::Float(0.1)),
                Type::owned(BaseType::F64),
            ),
            (
                Expression::Literal(Literal::Int(0)),
                Type::owned(BaseType::I64),
            ),
        ];

        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);

        for (expression, expected_type) in tests {
            let result = typechecker.check_expression(&expression).unwrap();
            assert_eq!(result, expected_type);
        }
    }
}
