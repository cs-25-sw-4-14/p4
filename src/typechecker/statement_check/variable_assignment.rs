use crate::typechecker::expression_check;
use crate::{
    ast::VariableAssignment,
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    declaration: &VariableAssignment,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    debugger: &crate::debugger::Debugger,
    l: usize,
) -> Result<()> {
    let expression_type = expression_check::check(&declaration.value, v_table, f_table, s_table)?;

    if let Some(var) = v_table.get(&declaration.target) {
        let var_type = &var.var_type;
        if var_type != &expression_type {
            return Err(anyhow::anyhow!(
                "{}: Type mismatch: expected {:?}, found {:?}",
                debugger.display(l),
                var_type,
                expression_type
            ));
        }
    } else {
        return Err(anyhow::anyhow!("Variable {} declared", declaration.target));
    }

    Ok(())
}
