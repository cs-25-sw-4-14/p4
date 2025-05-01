use crate::typechecker::expression_check;
use crate::{
    ast::Write,
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable, types::Type},
};
use anyhow::Result;

pub fn check(
    write_stmt: &Write,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    debugger: &crate::debugger::Debugger,
    l: usize, 
) -> Result<()> {
    // Check the expression for the write statement
    let value_type = expression_check::check(&write_stmt.value, v_table, f_table, s_table)?;
    if value_type != Type::String {
        return Err(anyhow::anyhow!(
            "write(...) expects a string expression, found {:?}",
            debugger.display(l),
            value_type
        ));
    }
    Ok(())
}
