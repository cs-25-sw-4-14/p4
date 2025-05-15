use crate::ast::{Type, VectorAccess};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_vector_access(&self, vector_access: &VectorAccess) -> Result<Type> {
        let index_type = self.check_expression(&vector_access.index)?;

        if index_type != Type::I64 {
            return Err(anyhow::anyhow!("Index must be of type I64"));
        }
        if let Some(var) = self.v_table.get(&vector_access.vector_name) {
            if let Type::Vector(inner_type) = &var.var_type {
                Ok(*inner_type.clone())
            } else {
                Err(anyhow::anyhow!(
                    "Variable {} is not a vector",
                    vector_access.vector_name
                ))
            }
        } else {
            Err(anyhow::anyhow!(
                "Variable {} not found in vtable",
                vector_access.vector_name
            ))

        }
    }
}

#[cfg(test)]
pub mod vector_access_tests {
    use super::*;
    use crate::ast::{Expression, Literal, VariableDeclaration, VectorLiteral};
    use crate::debugger::Debugger;

    #[test]
    fn test_vector_access() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        typechecker.v_table.variables.insert(
            "my_vector".to_string(),
            VariableDeclaration {
                trivial: None,
                mutable: None,
                var_type: Type::Vector(Box::new(Type::I64)),
                name: "my_vector".to_string(),
                value: Expression::VectorLiteral(Box::new(VectorLiteral {
                    elements: vec![Expression::Literal(Literal::Int(0))],
                })),
            },
        );
        let expression = Expression::VectorAccess(Box::new(VectorAccess {
            vector_name: "my_vector".to_string(),
            index: Expression::Literal(Literal::Int(0)),
        }));

        let result = typechecker.check_expression(&expression);
        assert!(result.is_ok());
    }
}
