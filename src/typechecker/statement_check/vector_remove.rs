

use crate::{
    ast::{VectorRemove, Type},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use crate::typechecker::expression_check;
use anyhow::Result;


pub fn check(
    vector_insert: &VectorRemove,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    _l: usize,

) -> Result<()> {

    let vector_name = vector_insert.vector_name.clone();
    let index = vector_insert.index.clone();

    // check if index is an expression
    let index_type = expression_check::check(&index, v_table, f_table, s_table)?;
    if index_type != Type::I64 {
        return Err(anyhow::anyhow!("Index must be of type I64"));
    }

    // check if vector is defined and has type vector
    if let Some(var) = v_table.get(&vector_name) {
        if let Type::Vector(inner_type) = &var.var_type {
            // return the type of the vector
            return Ok(());
        } else {
            return Err(anyhow::anyhow!(
                "Variable {} is not a vector",
                vector_name
            ));
        }
    } else {
        return Err(anyhow::anyhow!("Variable {} not found in vtable", vector_name));
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expression, Statement, VectorLiteral, VariableDeclaration, Literal};
    use crate::typechecker::statement_check;
    use crate::typechecker::variable_table::VTable;
    use crate::typechecker::function_table::FTable;
    use crate::typechecker::struct_table::STable;

    #[test]
    fn test_vector_remove() {
        let mut v_table = VTable::new();
        v_table.variables.insert(
            "my_vector".to_string(),
            VariableDeclaration {
                var_type: Type::Vector(Box::new(Type::I64)),
                name: "my_vector".to_string(),
                value: Expression::VectorLiteral(Box::new(VectorLiteral {
                    elements: vec![],
                })),
                trivial: None,
                mutable: None,
            },
        );

        let vector_remove = VectorRemove {
            vector_name: "my_vector".to_string(),
            index: Expression::Literal(Literal::Int(1)),
        };

        let result = check(&vector_remove, &mut v_table, &FTable::new(), &STable::new(), 0);
        assert!(result.is_ok());
    }
    }
