use crate::ast::{Type, VectorRemove};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_vector_remove(&mut self, vector_insert: &VectorRemove, _l: usize) -> Result<()> {
        let vector_name = vector_insert.vector_name.clone();
        let index = vector_insert.index.clone();

        // check if index is an expression
        let index_type = self.check_expression(&index)?;
        if index_type != Type::I64 {
            return Err(anyhow::anyhow!("Index must be of type I64"));
        }

        // check if vector is defined and has type vector
        if let Some(var) = self.v_table.get(&vector_name) {
            if let Type::Vector(_, _) = &var.variable_declaration.var_type {
                // return the type of the vector
                Ok(())
            } else {
                Err(anyhow::anyhow!("Variable {} is not a vector", vector_name))
            }
        } else {
            Err(anyhow::anyhow!(
                "Variable {} not found in vtable",
                vector_name
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast::{Expression, Literal, VariableDeclaration, VectorLiteral},
        debugger::Debugger,
        typechecker::variable_table::Inner,
    };

    #[test]
    fn test_vector_remove() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        typechecker.v_table.variables.insert(
            "my_vector".to_string(),
            Inner::new(VariableDeclaration {
                var_type: Type::Vector(Box::new(Type::I64), 1),
                name: "my_vector".to_string(),
                value: Expression::VectorLiteral(Box::new(VectorLiteral { elements: vec![] })),
                trivial: None,
                mutable: None,
            }),
        );

        let vector_remove = VectorRemove {
            vector_name: "my_vector".to_string(),
            index: Expression::Literal(Literal::Int(1)),
        };

        let result = typechecker.check_vector_remove(&vector_remove, 0);

        assert!(result.is_ok());
    }
}
