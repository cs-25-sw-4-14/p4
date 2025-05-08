use crate::ast::{Type, VectorRemove};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_vector_remove(&self, vector_insert: &VectorRemove, _l: usize) -> Result<()> {
        let vector_name = vector_insert.vector_name.clone();
        let index = vector_insert.index.clone();

<<<<<<< HEAD
    // check if index is an expression
    let index_type = expression_check::check(&index, v_table, f_table, s_table)?;
    if index_type != Type::I64 {
        return Err(anyhow::anyhow!("Index must be of type I64"));
    }

    // check if vector is defined and has type vector
    if let Some(var) = v_table.get(&vector_name) {
        if let Type::Vector(_inner_type, _) = &var.var_type {
            // return the type of the vector
            return Ok(());
        } else {
            return Err(anyhow::anyhow!("Variable {} is not a vector", vector_name));
=======
        // check if index is an expression
        let index_type = self.check_expression(&index)?;
        if index_type != Type::I64 {
            return Err(anyhow::anyhow!("Index must be of type I64"));
        }

        // check if vector is defined and has type vector
        if let Some(var) = self.v_table.get(&vector_name) {
            if let Type::Vector(_inner_type) = &var.var_type {
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
>>>>>>> main
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expression, Literal, VariableDeclaration, VectorLiteral};
<<<<<<< HEAD
    use crate::typechecker::function_table::FTable;
    use crate::typechecker::struct_table::STable;
    use crate::typechecker::variable_table::VTable;
=======
    use crate::debugger::Debugger;
>>>>>>> main

    #[test]
    fn test_vector_remove() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        typechecker.v_table.variables.insert(
            "my_vector".to_string(),
            VariableDeclaration {
                var_type: Type::Vector(Box::new(Type::I64), 0),
                name: "my_vector".to_string(),
                value: Expression::VectorLiteral(Box::new(VectorLiteral { elements: vec![] })),
                trivial: None,
                mutable: None,
            },
        );

        let vector_remove = VectorRemove {
            vector_name: "my_vector".to_string(),
            index: Expression::Literal(Literal::Int(1)),
        };

        let result = typechecker.check_vector_remove(&vector_remove, 0);

        assert!(result.is_ok());
    }
}
