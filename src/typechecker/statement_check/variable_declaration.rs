use crate::typechecker::expression_check;
use crate::{
    ast::VariableDeclaration,
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    declaration: &VariableDeclaration,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    debugger: &crate::debugger::Debugger,
    l: usize,
) -> Result<()> {
    // Check if the type of the value matches the declared type
    let value_type = expression_check::check(&declaration.value, v_table, f_table, s_table)?;
    if value_type != declaration.var_type {
        return Err(anyhow::anyhow!(
            "{}: Type mismatch: expected {:?}, found {:?}",
            debugger.display(l),
            declaration.var_type,
            value_type
        ));
    }

    semantic_check_variable_already_declared(declaration, v_table)?;

    v_table
        .variables
        .insert(declaration.name.clone(), declaration.clone());

    Ok(())
}

pub fn semantic_check_variable_already_declared<'a>(
    declaration: &'a VariableDeclaration,
    v_table: &'a VTable,
) -> Result<()> {
    // Check if the variable is already declared in the current scope
    if v_table.variables.contains_key(&declaration.name) {
        return Err(anyhow::anyhow!(
            "Variable {} is already declared",
            declaration.name
        ));
    }
    Ok(())
}
