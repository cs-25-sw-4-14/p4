use super::{function_table::FTable, struct_table::STable, variable_table::VTable};
use crate::ast::Statement;
use anyhow::Result;

mod array_assignment;
mod function_call;
mod if_else;
mod throwaway;
mod variable_assignment;
mod variable_declaration;
mod while_statement;

pub fn check(
    expression: &Statement,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    debugger: &crate::debugger::Debugger,
) -> Result<()> {
    match expression {
        Statement::Throwaway(expression, l) => {
            throwaway::check(
                expression,
                v_table, f_table, s_table, debugger, *l,
            )?;
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
        } // check if the variable is declared and if the type matches
        Statement::Assignment(variable_assignment, l) => {
            variable_assignment::check(
                variable_assignment,
                v_table,
                f_table,
                s_table,
                debugger,
                *l,
            )?;
        } //check if the variable is declared and if the type matches
        Statement::ArrayAssignment(_array_assignment, l) => {
            array_assignment::check(_array_assignment, v_table, f_table, s_table, *l)?;
        } // check if the struct is declared
        Statement::If(if_else, _) => {
            if_else::check(if_else, v_table, f_table, s_table, debugger)?;
        }
        Statement::While(while_statement, _) => {
            while_statement::check(while_statement, v_table, f_table, s_table, debugger)?;
        } // check if condition is bool
        Statement::Break(_) => {}
        Statement::Continue(_) => {}
        Statement::Return(_, _) => {}
        Statement::FunctionCall(function_call, _) => {
            function_call::check(function_call, v_table, f_table, s_table)?;
        } // check if function is defined, type is void
        Statement::Print(_, _) => {}
        Statement::Write(_, _) => {}
    }

    Ok(())
}
