use crate::typechecker::expression_check;
use crate::{
    ast::{Type, VectorInsert},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    vector_insert: &VectorInsert,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    _l: usize,
) -> Result<()> {
    let vector_name = vector_insert.vector_name.clone();
    let index = vector_insert.index.clone();
    let value = vector_insert.value.clone();

    // check if index is an expression
    let index_type = expression_check::check(&index, v_table, f_table, s_table)?;
    if index_type != Type::I64 {
        return Err(anyhow::anyhow!("Index must be of type I64"));
    }

    // check if value is an expression
    let value_type = expression_check::check(&value, v_table, f_table, s_table)?;

    // check if vector is defined and has type vector
    if let Some(var) = v_table.get(&vector_name) {
        if let Type::Vector(inner_type, _) = &var.var_type {
            if *inner_type != Box::new(value_type.clone()) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Expression, Literal, Type, VariableDeclaration, VectorLiteral};

    use crate::typechecker::variable_table::VTable;

    #[test]
    fn test_vector_insert() {
        let mut v_table = VTable::new();
        v_table.variables.insert(
            "my_vector".to_string(),
            VariableDeclaration {
                var_type: Type::Vector(Box::new(Type::I64), 0),
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

        let result = check(
            &vector_insert,
            &mut v_table,
            &FTable::new(),
            &STable::new(),
            0,
        );
        assert!(result.is_ok());
    }
}
