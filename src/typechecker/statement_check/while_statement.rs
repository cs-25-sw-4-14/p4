use crate::{ast::{ Type, WhileStatement}, typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;


pub fn check(declaration: &WhileStatement, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<()> {	

    let condition_type = expression_check::check(&declaration.condition, v_table, f_table, s_table)?;
    if condition_type != Type::Bool {
        return Err(anyhow::anyhow!("Condition type is not bool: {:?}", condition_type));
    }
    // Check the body of the while statement
    for statement in &declaration.body {
        super::check(statement, v_table, f_table, s_table)?;
    }
    
    
    Ok(())
}