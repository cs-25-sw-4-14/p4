use crate::ast::Statement;
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use anyhow::Result;

// Import submodules
pub mod variable_statements;
pub mod control_flow;
pub mod function_statements;
pub mod utils;

/// Main entry point for statement type checking
pub fn check(
    statement: &Statement, 
    vtable: &VTable, 
    ftable: &Ftable,
    stable: &STable
) -> Result<()> {
    match statement {
        Statement::VariableDeclaration(var_decl) => 
            variable_statements::check_var_declaration(var_decl, vtable, ftable, stable),
            
        Statement::Assignment(var_assign) =>
            variable_statements::check_assignment(var_assign, vtable, ftable, stable),
            
        Statement::If(if_stmt) =>
            control_flow::check_if_statement(if_stmt, vtable, ftable, stable),
            
        Statement::While(while_stmt) =>
            control_flow::check_while_statement(while_stmt, vtable, ftable, stable),
            
        Statement::Break =>
            control_flow::check_break(),
            
        Statement::Continue =>
            control_flow::check_continue(),
            
        Statement::Return(return_stmt) =>
            function_statements::check_return(return_stmt, vtable, ftable, stable),
            
        Statement::FunctionCall(func_call) =>
            function_statements::check_function_call(func_call, vtable, ftable, stable),
    }
}
