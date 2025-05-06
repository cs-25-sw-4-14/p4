use crate::ast::{Write, Type}; 
use crate::typechecker::{
    expression_check, 
    function_table::FTable,
    struct_table::STable,
    variable_table::VTable,
};
use anyhow::{anyhow, Result}; 

pub fn check(
    write_expr: &Write, 
    v_table: &VTable, 
    f_table: &FTable,
    s_table: &STable,  
) -> Result<Type> {
    // Check the type of the 'value' expression (the content to write)
    let value_type = expression_check::check(&write_expr.value, v_table, f_table, s_table)?;

    // Validate that the value expression evaluates to a String
    if value_type != Type::String {

        return Err(anyhow!(
            "Type Error: `write_file` expects a String expression for the 'value' argument (content), but found type '{:?}'",
            value_type
        ));
    }

    // Check the type of the 'target' expression (the filename)
    let target_type = expression_check::check(&write_expr.target, v_table, f_table, s_table)?;

    // Validate that the target expression evaluates to a String
    if target_type != Type::String {

        return Err(anyhow!(
            "Type Error: `write_file` expects a String expression for the 'target' argument (filename), but found type '{:?}'",
            target_type
        ));
    }

    Ok(Type::I32)
}