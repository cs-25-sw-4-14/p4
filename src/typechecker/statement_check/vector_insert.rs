use crate::ast::{Type, VectorInsert};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_vector_insert(&self, vector_insert: &VectorInsert, _l: usize) -> Result<()> {
        let vector_name = vector_insert.vector_name.clone();
        let index = vector_insert.index.clone();
        let value = vector_insert.value.clone();

        // check if index is an expression
        let index_type = self.check_expression(&index)?;
        if index_type != Type::I64 {
            return Err(anyhow::anyhow!("Index must be of type I64"));
        }

        // check if value is an expression
        let value_type = self.check_expression(&value)?;

        // check if vector is defined and has type vector
        if let Some(var) = self.v_table.get(&vector_name) {
            if let Type::Vector(inner_type) = &var.var_type {
                if **inner_type != value_type {
                    return Err(anyhow::anyhow!(
                        "Value type {:?} does not match vector type {:?}",
                        value_type,
                        inner_type
                    ));
                }
            } else {
                return Err(anyhow::anyhow!("Variable {} is not a vector", vector_name));
            }
        } else {
            return Err(anyhow::anyhow!(
                "Variable {} not found in vtable",
                vector_name
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expression, Literal, Type, VariableDeclaration, VectorLiteral};

    use crate::debugger::Debugger;
    

    #[test]
    fn test_vector_insert() {
        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        typechecker.v_table.variables.insert(
            "my_vector".to_string(),
            VariableDeclaration {
                var_type: Type::Vector(Box::new(Type::I64)),
                name: "my_vector".to_string(),
                value: Expression::VectorLiteral(Box::new(VectorLiteral { elements: vec![] })),
                trivial: None,
                mutable: None,
            },
        );

        let vector_insert = VectorInsert {
            vector_name: "my_vector".to_string(),
            index: Expression::Literal(Literal::Int(1)),
            value: Expression::Literal(Literal::Int(52)),
        };

        typechecker.check_vector_insert(&vector_insert, 0).unwrap();
    }
}
