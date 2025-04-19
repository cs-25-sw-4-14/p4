use super::{function_table::FTable, struct_table::STable, variable_table::VTable};
use crate::ast::{Statement, Type};
use anyhow::Result;

mod variable_assignment;

mod function_call;
mod if_else;
mod variable_declaration;
mod while_statement;

pub fn check(
    expression: &Statement,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<()> {
    match expression {
        Statement::VariableDeclaration(_variable_declaration) => {
            variable_declaration::check(_variable_declaration, v_table, f_table, s_table)?;
        }
        Statement::Assignment(variable_assignment) => {
            variable_assignment::check(variable_assignment, v_table, f_table, s_table)?;
        } //check if the variable is declared and if the type matches
        Statement::If(if_else) => {
            if_else::check(if_else, v_table, f_table, s_table)?;
        } // check if condition is bool, and there is no break or continue
        Statement::While(while_statement) => {
            while_statement::check(while_statement, v_table, f_table, s_table)?;
        } // check if condition is bool
        Statement::Break => {}
        Statement::Continue => {}
        Statement::Return(_) => {}
        Statement::FunctionCall(function_call) => {
            function_call::check(function_call, v_table, f_table, s_table)?;
        } // check if function is defined, type is void
    };

    Ok(())
}
