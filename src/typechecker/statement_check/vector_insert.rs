

use crate::{
    ast::{VectorInsert, Type},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use crate::typechecker::expression_check;
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
        if let Type::Vector(inner_type) = &var.var_type {
            if *inner_type != Box::new(value_type.clone()) {
                return Err(anyhow::anyhow!(
                    "Value type {:?} does not match vector type {:?}",
                    value_type,
                    inner_type
                ));
            }
        } else {
            return Err(anyhow::anyhow!(
                "Variable {} is not a vector",
                vector_name
            ));
        }
    } else {
        return Err(anyhow::anyhow!("Variable {} not found in vtable", vector_name));
    }


    Ok(())



}
