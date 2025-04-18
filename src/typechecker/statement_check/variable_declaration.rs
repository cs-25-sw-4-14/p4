use crate::typechecker::expression_check;
use crate::{
    ast::{Type, VariableDeclaration},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    declaration: &VariableDeclaration,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<()> {
    // Check if the type of the value matches the declared type
    let value_type = expression_check::check(&declaration.value, v_table, f_table, s_table)?;
    if value_type != declaration.var_type {
        return Err(anyhow::anyhow!(
            "Type mismatch: expected {:?}, found {:?}",
            declaration.var_type,
            value_type
        ));
    }

    // TODO add the variable to the v table ??

    Ok(())
}
