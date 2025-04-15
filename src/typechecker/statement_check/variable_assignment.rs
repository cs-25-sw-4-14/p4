use crate::{ast::{ Type, VariableAssignment}, typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable}};
use anyhow::Result;
use crate::typechecker::expression_check;


pub fn check(declaration: &VariableAssignment, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<()> {	
    
    let expression_type = expression_check::check(&declaration.value, v_table, f_table, s_table)?;
    let name_type =  v_table.get(&declaration.target);
    if expression_type !=name_type {
        return Err(anyhow::anyhow!("Type mismatch: expected {:?}, found {:?}", expression_type, name_type));
    }

    // TODO add the variable to the v table ?? 

    Ok(())
    }

