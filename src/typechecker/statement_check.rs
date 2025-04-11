
use crate::ast::Statement;
use super::{function_table::Ftable, variable_table::VTable};
use anyhow::Result;

pub fn check(expression: &Statement, _vtable: &VTable, _ftable: &Ftable) -> Result<()> {

    match expression {
        Statement::VariableDeclaration(_variable_declaration) => todo!(),
        Statement::Assignment(_variable_assignment) => todo!(),
        Statement::If(_if_statement) => todo!(),
        Statement::While(_while_statement) => todo!(),
        Statement::Break => todo!(),
        Statement::Continue => todo!(),
        Statement::Return(_return_statement) => todo!(),
        Statement::FunctionCall(_function_call) => todo!(),
    }
}
