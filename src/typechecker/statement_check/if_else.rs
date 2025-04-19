use crate::typechecker::expression_check;
use crate::typechecker::statement_check::Statement;
use crate::{
    ast::{IfStatement, Type},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    declaration: &IfStatement,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<()> {
    let condition_type =
        expression_check::check(&declaration.condition, v_table, f_table, s_table)?;
    if condition_type != Type::Bool {
        return Err(anyhow::anyhow!(
            "Condition type is not bool: {:?}",
            condition_type
        ));
    }
    // Check the body of the if statement
    for statement in &declaration.then_block {
        super::check(statement, v_table, f_table, s_table)?;
    }
    // Check the else if blocks
    for else_if in &declaration.else_if_blocks {
        let condition_type =
            expression_check::check(&else_if.condition, v_table, f_table, s_table)?;
        if condition_type != Type::Bool {
            return Err(anyhow::anyhow!(
                "Condition type is not bool: {:?}",
                condition_type
            ));
        }
        for statement in &else_if.body {
            super::check(statement, v_table, f_table, s_table)?;
        }
    }
    // Check the else block if it exists
    if let Some(else_block) = &declaration.else_block {
        for statement in else_block {
            super::check(statement, v_table, f_table, s_table)?;
        }
    }

    // Check for break or continue statements in the if statement
    for statement in &declaration.then_block {
        if let Statement::Break | Statement::Continue = statement {
            return Err(anyhow::anyhow!(
                "Break or continue statement found in if statement."
            ));
        }
    }
    // Check for break or continue statements in the else if blocks
    for else_if in &declaration.else_if_blocks {
        for statement in &else_if.body {
            if let Statement::Break | Statement::Continue = statement {
                return Err(anyhow::anyhow!(
                    "Break or continue statement found in else if block."
                ));
            }
        }
    }
    // Check for break or continue statements in the else block if it exists
    if let Some(else_block) = &declaration.else_block {
        for statement in else_block {
            if let Statement::Break | Statement::Continue = statement {
                return Err(anyhow::anyhow!(
                    "Break or continue statement found in else block."
                ));
            }
        }
    }

    Ok(())
}
