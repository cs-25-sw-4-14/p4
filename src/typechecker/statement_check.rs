

use super::{function_table::FTable, variable_table::VTable, struct_table::STable};
use anyhow::Result;
use crate::ast::{Type, Statement};


mod variable_declaration;
mod return_statment;

pub fn check(expression: &Statement, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {

    match expression {
        Statement::VariableDeclaration(_variable_declaration) => variable_declaration::check(_variable_declaration, v_table, f_table, s_table),
        Statement::Assignment(_variable_assignment) => todo!(),
        Statement::If(_if_statement) => todo!(),
        Statement::While(_while_statement) => todo!(),
        Statement::Break => todo!(),
        Statement::Continue => todo!(),
        Statement::Return(return_statement) => return_statment::check(return_statement, v_table, f_table, s_table),
        Statement::FunctionCall(_function_call) => todo!(),
    }
}
