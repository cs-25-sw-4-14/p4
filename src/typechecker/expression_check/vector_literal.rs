use crate::ast::{Type, VectorLiteral};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_vector_literal(&self, vector_literal: &VectorLiteral) -> Result<Type> {
        let vector_type =
            self.semantic_check_all_elements_same_type_vector_literal(vector_literal)?;

        Ok(vector_type)
    }

    pub fn semantic_check_all_elements_same_type_vector_literal(
        &self,
        vector_literal: &VectorLiteral,
    ) -> Result<Type> {
        // Allow empty vectors
        if vector_literal.elements.is_empty() {
            return Ok(Type::Vector(Box::new(Type::I64), 0));
        }
        let first_type = self.check_expression(&vector_literal.elements[0])?;

        for element in &vector_literal.elements[1..] {
            let element_type = self.check_expression(element)?;
            if element_type != first_type {
                return Err(anyhow::anyhow!(
                    "Vector elements have different types: {:?} and {:?}",
                    first_type,
                    element_type
                ));
            }
        }
        Ok(Type::Vector(Box::new(first_type), vector_literal.elements.len() as u64))
    }
}

#[cfg(test)]
pub mod vector_literal_tests {
    use super::*;
    use crate::ast::{Expression, Literal};
    use crate::debugger::Debugger;

    #[test]
    fn test_vector_literal() {
        let expression = Expression::VectorLiteral(Box::new(VectorLiteral {
            elements: vec![
                Expression::Literal(Literal::Int(1)),
                Expression::Literal(Literal::Int(2)),
                Expression::Literal(Literal::Int(3)),
            ],
        }));

        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);

        let result = typechecker.check_expression(&expression);
        assert!(result.is_ok());
    }
}
