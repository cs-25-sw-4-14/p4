use crate::typechecker::expression_check;
use crate::{
    ast::Read,
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable, types::Type},
};
use anyhow::Result;

pub fn check(
    read_file: &Read,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    debugger: &crate::debugger::Debugger,
    l: usize, 
) -> Result<()> {
    let value_type = expression_check::check(&read_file.value, v_table, f_table, s_table)?;
    
    if value_type != Type::String {
        return Err(anyhow::anyhow!(
            "{}: read_file(...) expects a string expression, found {:?}",
            debugger.display(l),
            value_type
        ));
    }

    Ok(())
}
