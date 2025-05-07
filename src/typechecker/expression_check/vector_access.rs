use crate::typechecker::expression_check;
use crate::{
    ast::{Type, VectorAccess},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    vector_access: &VectorAccess,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<Type> {
    //check if index is an expression
    let index_type = expression_check::check(&vector_access.index, v_table, f_table, s_table)?;
    if index_type != Type::I64 {
        return Err(anyhow::anyhow!("Index must be of type I64"));
    }
    //check if vector is defined and has type vector
    if let Some(var) = v_table.get(&vector_access.vector_name) {
        if let Type::Vector(inner_type, len) = &var.var_type {
            // return the type of the vector
            return Ok(*inner_type.clone());
        } else {
            return Err(anyhow::anyhow!(
                "Variable {} is not a vector",
                vector_access.vector_name
            ));
        }
    } else {
        Err(anyhow::anyhow!(
            "Variable {} not found in vtable",
            vector_access.vector_name
        ))
    }
}

#[cfg(test)]
pub mod vector_access_tests {
    use super::*;
    use crate::ast::{Expression, Literal, VariableDeclaration, VectorLiteral};
    use crate::typechecker::expression_check;

    #[test]
    fn test_vector_access() {
        let mut v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        v_table.variables.insert(
            "my_vector".to_string(),
            VariableDeclaration {
                trivial: None,
                mutable: None,
                var_type: Type::Vector(Box::new(Type::I64), 0),
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

        let result = expression_check::check(&expression, &v_table, &f_table, &s_table);
        assert!(result.is_ok());
    }
}
