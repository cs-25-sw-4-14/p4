
use crate::ast::Statement;
use super::{function_table::Ftable, variable_table::VTable};
use anyhow::Result;

pub fn check(expression: &Statement, vtable: &VTable, ftable: &Ftable) -> Result<()> {

    match expression {
        Statement::VariableDeclaration(variable_declaration) => todo!(),
        Statement::Assignment(variable_assignment) => todo!(),
        Statement::If(if_statement) => todo!(),
        Statement::While(while_statement) => todo!(),
        Statement::Break => todo!(),
        Statement::Continue => todo!(),
        Statement::Return(return_statement) => todo!(),
        Statement::FunctionCall(function_call) => todo!(),
    }
}
