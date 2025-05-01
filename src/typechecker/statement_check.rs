use super::{
    expression_check, function_table::FTable, struct_table::STable, variable_table::VTable,
};
use crate::ast::Statement;
use anyhow::Result;

mod assignment;
mod function_call;
mod if_else;
mod throwaway;
mod variable_declaration;
mod while_statement;
mod write;
mod read;

pub fn check(
    expression: &Statement,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    debugger: &crate::debugger::Debugger,
) -> Result<()> {
    match expression {
        Statement::Throwaway(expression, l) => {
            throwaway::check(expression, v_table, f_table, s_table, debugger, *l)?;
        }
        Statement::VariableDeclaration(variable_declaration, l) => {
            variable_declaration::check(
                variable_declaration,
                v_table,
                f_table,
                s_table,
                debugger,
                *l,
            )?;
        }
        Statement::Assignment(variable_assignment, l) => {
            assignment::check(variable_assignment, v_table, f_table, s_table, debugger, *l)?;
        }
        Statement::If(if_else, _) => {
            if_else::check(if_else, v_table, f_table, s_table, debugger)?;
        }
        Statement::While(while_statement, _) => {
            while_statement::check(while_statement, v_table, f_table, s_table, debugger)?;
        }
        Statement::Break(_) => {}
        Statement::Continue(_) => {}
        Statement::Return(_, _) => {}
        Statement::FunctionCall(function_call, _) => {
            function_call::check(function_call, v_table, f_table, s_table)?;
        }
        Statement::Print(print, _) => {
            expression_check::check(&print.expression, v_table, f_table, s_table)?;
        }
        Statement::Write(_, _) => {}
    }

    Ok(())
}
