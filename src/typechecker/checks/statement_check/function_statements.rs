use crate::ast::{FunctionCall, ReturnStatement, Expression, Type};
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use crate::typechecker::checks::expression_check;
use anyhow::Result;

/// Type check a function call statement
pub fn check_function_call(
    function_call: &FunctionCall,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<()> {
    // Check the function call expression
    let _result_type = expression_check::check(
        &Expression::FunctionCall(function_call.clone()),
        vtable, ftable, stable
    )?;

    // We don't need to do anything with the result_type
    // since we're just calling the function for its side effects
    Ok(())
}

/// Type check a return statement
pub fn check_return(
    return_statement: &ReturnStatement,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<()> {
    // In a full implementation, we would check if the return type matches the function's return type
    // For now, just check that the expression is well-typed if present
    if let Some(expr) = &return_statement.value {
        let _ = expression_check::check(expr, vtable, ftable, stable)?;
    }
    
    Ok(())
}

