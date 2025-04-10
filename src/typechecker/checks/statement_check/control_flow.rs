use crate::ast::{IfStatement, WhileStatement, Type, Expression, ElseIf, Statement};
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use crate::typechecker::checks::expression_check;
use super::super::statement_check;
use anyhow::Result;

/// Type check an if statement
pub fn check_if_statement(
    if_statement: &IfStatement,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<()> {
    // Check that the condition is a boolean
    let condition_type = expression_check::check(&if_statement.condition, vtable, ftable, stable)?;
    if condition_type != Type::Bool {
        return Err(anyhow::anyhow!(
            "If condition must be a boolean, got {:?}",
            condition_type
        ));
    }
    
    // Check the then block
    for stmt in &if_statement.then_block {
        statement_check::check(stmt, vtable, ftable, stable)?;
    }
    
    // Check all else-if blocks
    for else_if in &if_statement.else_if_blocks {
        // Check the condition
        let else_if_condition_type = expression_check::check(&else_if.condition, vtable, ftable, stable)?;
        if else_if_condition_type != Type::Bool {
            return Err(anyhow::anyhow!(
                "Else-if condition must be a boolean, got {:?}",
                else_if_condition_type
            ));
        }
        
        // Check the body
        for stmt in &else_if.body {
            statement_check::check(stmt, vtable, ftable, stable)?;
        }
    }
    
    // Check the else block if present
    if let Some(else_block) = &if_statement.else_block {
        for stmt in else_block {
            statement_check::check(stmt, vtable, ftable, stable)?;
        }
    }
    
    Ok(())
}

/// Type check a while statement
pub fn check_while_statement(
    while_statement: &WhileStatement,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<()> {
    // Check that the condition is a boolean
    let condition_type = expression_check::check(&while_statement.condition, vtable, ftable, stable)?;
    if condition_type != Type::Bool {
        return Err(anyhow::anyhow!(
            "While condition must be a boolean, got {:?}",
            condition_type
        ));
    }
    
    // Check the loop body
    for stmt in &while_statement.body {
        statement_check::check(stmt, vtable, ftable, stable)?;
    }
    
    Ok(())
}

/// Type check a break statement
pub fn check_break() -> Result<()> {
    // No type checking needed for break
    // In a more advanced type checker, we might check if it's inside a loop
    Ok(())
}

/// Type check a continue statement
pub fn check_continue() -> Result<()> {
    // No type checking needed for continue
    // In a more advanced type checker, we might check if it's inside a loop
    Ok(())
}

