use crate::ast::{Read, Type}; 
use crate::typechecker::{
    expression_check, 
    function_table::FTable,
    struct_table::STable,
    variable_table::VTable,
};
use anyhow::{anyhow, Result}; 


pub fn check(
    read_expr: &Read,
    v_table: &VTable,
    f_table: &FTable, 
    s_table: &STable, 
) -> Result<Type> {
    // Check the type of the expression provided as the filename argument
    let filename_type = expression_check::check(&read_expr.value, v_table, f_table, s_table)?;

    // Validate that the filename expression evaluates to a String
    if filename_type != Type::String {

        return Err(anyhow!(
            "Type Error: `read_file` expects a String expression for the filename, but found type '{:?}'",
            filename_type
        ));
    }

    Ok(Type::String)
}