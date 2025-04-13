

use super::{function_table::FTable, variable_table::VTable, struct_table::STable};
use anyhow::Result;
use crate::ast::{Type, Statement};


mod variable_declaration;

pub fn check(expression: &Statement, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<()> {

    match expression {
        Statement::VariableDeclaration(_variable_declaration) => {variable_declaration::check(_variable_declaration, v_table, f_table, s_table)?;},
        Statement::Assignment(_variable_assignment) => todo!(), //check if the variable is declared and if the type matches
        Statement::If(_if_statement) => todo!(), // check if condition is bool, and there is no break or continue 
        Statement::While(_while_statement) => todo!(), // check if condition is bool
        Statement::Break =>{},
        Statement::Continue => {},
        Statement::Return(_) => {},
        Statement::FunctionCall(_function_call) => todo!(), // check if function is defined, type is void 
    };

    Ok(())
}
