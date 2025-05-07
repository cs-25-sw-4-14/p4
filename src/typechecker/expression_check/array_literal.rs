use crate::ast::{ArrayLiteral, Type};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl<'a> Typechecker<'a> {
    pub fn check_array_literal(&self, array_literal: &Box<ArrayLiteral>) -> Result<Type> {
        self.semantic_check_literal_not_empty(array_literal)?;

        let array_type = self.semantic_check_all_elements_same_type(array_literal)?;

        Ok(Type::Array(
            Box::new(array_type.clone()),
            array_literal.elements.len() as u64,
        ))
    }

    pub fn semantic_check_literal_not_empty(
        &self,
        array_literal: &Box<ArrayLiteral>,
    ) -> Result<()> {
        if array_literal.elements.is_empty() {
            return Err(anyhow::anyhow!("Array literal is empty"));
        }
        Ok(())
    }

    pub fn semantic_check_all_elements_same_type(
        &self,
        array_literal: &'a Box<ArrayLiteral>,
    ) -> Result<Type> {
        let first_type = self.check_expression(&array_literal.elements[0])?;

        for element in &array_literal.elements[1..] {
            let element_type = self.check_expression(element)?;
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
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{ArrayLiteral, Expression, Literal};
    use crate::debugger::Debugger;

    #[test]
    fn check_array_literal_should_return_type() {
        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);

        let expression = Expression::ArrayLiteral(Box::new(ArrayLiteral {
            elements: vec![
                Expression::Literal(Literal::Int(1)),
                Expression::Literal(Literal::Int(2)),
                Expression::Literal(Literal::Int(3)),
            ],
        }));

        let result = typechecker.check_expression(&expression).unwrap();
        assert_eq!(result, Type::Array(Box::new(Type::I64), 3));
    }

    #[test]
    fn check_array_literal_should_return_type_mixed() {
        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);

        let expression = Expression::ArrayLiteral(Box::new(ArrayLiteral {
            elements: vec![
                Expression::Literal(Literal::Int(1)),
                Expression::Literal(Literal::Float(2.0)),
                Expression::Literal(Literal::Int(3)),
            ],
        }));

        typechecker.check_expression(&expression).unwrap_err();
    }

    #[test]
    fn check_array_literal_empty() {
        let debugger = Debugger::new(String::from(""));
        let typechecker = Typechecker::new(&debugger);

        let expression = Expression::ArrayLiteral(Box::new(ArrayLiteral { elements: vec![] }));

        typechecker.check_expression(&expression).unwrap_err();
    }
}
