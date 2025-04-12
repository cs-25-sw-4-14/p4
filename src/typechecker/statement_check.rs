
use crate::ast::Statement;
use super::{function_table::FTable, variable_table::VTable, struct_table::STable};
use anyhow::Result;
use crate::ast::Type;


mod variable_declaration;

pub fn check(expression: &Statement, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {

    match expression {
        Statement::VariableDeclaration(variable_declaration) => variable_declaration::check(variable_declaration, v_table, f_table, s_table),
        Statement::Assignment(_variable_assignment) => todo!(),
        Statement::If(_if_statement) => todo!(),
        Statement::While(_while_statement) => todo!(),
        Statement::Break => todo!(),
        Statement::Continue => todo!(),
        Statement::Return(_return_statement) => todo!(),
        Statement::FunctionCall(_function_call) => todo!(),
    }
}
