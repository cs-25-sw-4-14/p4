use crate::ast::{FunctionCall, Type, Expression};
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use super::super::expression_check;
use super::type_utils::{types_match, can_convert_literal};
use anyhow::Result;

/// Type check a function call expression
pub fn check_function_call(
    function_call: &FunctionCall,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<Type> {
    // Look up the function in the function table
    let function_entry = match ftable.get(&function_call.name) {
        Some(entry) => entry,
        None => return Err(anyhow::anyhow!("Function {} not found", function_call.name)),
    };
    
    // Get the function definition
    let function_def = &function_entry.def;
    
    // Check if the number of arguments matches the number of parameters
    if function_call.args.len() != function_def.params.len() {
        return Err(anyhow::anyhow!(
            "Function {} called with {} arguments, but it requires {} parameters",
            function_call.name,
            function_call.args.len(),
            function_def.params.len()
        ));
    }
    
    // Check the type of each argument
    for (i, (arg, param)) in function_call.args.iter().zip(function_def.params.iter()).enumerate() {
        let arg_type = expression_check::check(arg, vtable, ftable, stable)?;
        
        // Special handling for literal arguments
        if let Expression::Literal(literal) = arg {
            if can_convert_literal(literal, &param.param_type) {
                continue; // Literal value can be converted to the parameter type
            }
        }
        
        if !types_match(&arg_type, &param.param_type) {
            return Err(anyhow::anyhow!(
                "Type mismatch in argument {} of function {}: expected {:?}, got {:?}",
                i + 1,
                function_call.name,
                param.param_type,
                arg_type
            ));
        }
    }
    
    // Return the function's return type
    match &function_def.return_type {
        Some(return_type) => Ok(return_type.clone()),
        None => Ok(Type::Custom("Void".to_string())), // Assuming Void is represented as a custom type
    }
}

