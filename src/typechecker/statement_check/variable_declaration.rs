use crate::{ast::{StructFieldAccess, Type, VariableDeclaration}, typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;

pub fn check(declaration: &VariableDeclaration, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {
    // Check if the variable is already declared
    if Some (v_table.get(&declaration.name)).is_some() {
        return Err(anyhow::anyhow!("Variable {} already declared", declaration.name));
    }


    // Check if the type of the value matches the declared type
    let value_type = expression_check::check(&declaration.value, v_table, f_table, s_table)?;
    if value_type != declaration.var_type {
        return Err(anyhow::anyhow!("Type mismatch: expected {:?}, found {:?}", declaration.var_type, value_type));
    }

    // Add the variable to the variable table
    v_table.insert(declaration.name.clone(), declaration.var_type);

    Ok(value_type)
}