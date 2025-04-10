use crate::ast::{VariableDeclaration, VariableAssignment, Expression, Type, FunctionCall};
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use crate::typechecker::checks::expression_check;
use anyhow::Result;

/// Type check a variable declaration statement
pub fn check_var_declaration(
    variable_declaration: &VariableDeclaration,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<()> {
    // Special case for literal values
    if let Expression::Literal(literal) = &variable_declaration.value {
        // Check if the literal value can be converted to the variable's type
        if expression_check::can_convert_literal(literal, &variable_declaration.var_type) {
            return Ok(());
        }
    }
    
    // Special case for function calls to check return type compatibility
    if let Expression::FunctionCall(func_call) = &variable_declaration.value {
        check_function_return_type(func_call, &variable_declaration.var_type, ftable)?;
    }
    
    // Check that the expression has the expected type
    let expr_type = expression_check::check(&variable_declaration.value, vtable, ftable, stable)?;
    
    // Check if the declared type matches the expression type
    if !expression_check::types_match(&expr_type, &variable_declaration.var_type) {
        return Err(anyhow::anyhow!(
            "Type mismatch in variable declaration: {} is declared as {:?} but initialized with expression of type {:?}",
            variable_declaration.name, variable_declaration.var_type, expr_type
        ));
    }
    
    Ok(())
}

/// Type check a variable assignment statement
pub fn check_assignment(
    variable_assignment: &VariableAssignment,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<()> {
    // Check that the variable exists
    let var_type = match vtable.get(&variable_assignment.target) {
        Some(var) => {
            // Check if the variable is mutable
            if var.mutable.is_none() && var.trivial.is_none() {
                return Err(anyhow::anyhow!(
                    "Cannot assign to immutable variable {}",
                    variable_assignment.target
                ));
            }
            var.var_type.clone()
        },
        None => return Err(anyhow::anyhow!(
            "Assignment to undeclared variable {}",
            variable_assignment.target
        )),
    };
    
    // Special case for literal values
    if let Expression::Literal(literal) = &variable_assignment.value {
        // Check if the literal value can be converted to the variable's type
        if expression_check::can_convert_literal(literal, &var_type) {
            return Ok(());
        }
    }
    
    // Special case for function calls to check return type compatibility
    if let Expression::FunctionCall(func_call) = &variable_assignment.value {
        check_function_return_type(func_call, &var_type, ftable)?;
    }
    
    // Check that the expression has the expected type
    let expr_type = expression_check::check(&variable_assignment.value, vtable, ftable, stable)?;
    
    // Check if the types match
    if !expression_check::types_match(&expr_type, &var_type) {
        return Err(anyhow::anyhow!(
            "Type mismatch in assignment: {} has type {:?} but assigned expression has type {:?}",
            variable_assignment.target, var_type, expr_type
        ));
    }
    
    Ok(())
}

/// Helper function to check if a function's return type is compatible with the expected type
fn check_function_return_type(
    function_call: &FunctionCall,
    expected_type: &Type,
    ftable: &Ftable
) -> Result<()> {
    // Look up the function in the function table
    let function_entry = match ftable.get(&function_call.name) {
        Some(entry) => entry,
        None => return Ok(()), // Not found, but this will be caught elsewhere
    };
    
    // Get the function definition
    let function_def = &function_entry.def;
    
    // Check if the function has a return type
    match &function_def.return_type {
        Some(return_type) => {
            // Check if return type is compatible with expected type
            if !expression_check::types_match(return_type, expected_type) {
                return Err(anyhow::anyhow!(
                    "Function {} returns type {:?} which cannot be assigned to {:?}",
                    function_call.name, return_type, expected_type
                ));
            }
        },
        None => {
            // This is a void function, but we're expecting a value
            return Err(anyhow::anyhow!(
                "Cannot use void function {} in an expression that requires a value",
                function_call.name
            ));
        }
    }
    
    Ok(())
}


