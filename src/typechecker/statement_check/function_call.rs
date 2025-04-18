use crate::typechecker::expression_check;
use crate::{
    ast::{FunctionCall, Type},
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    declaration: &FunctionCall,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<()> {
    // Check if the function is defined
    if let Some(function) = f_table.get(&declaration.name) {
        // Check if the function is void
        if function.def.return_type.is_some() {
            return Err(anyhow::anyhow!(
                "Function {} is not void (has a return type)",
                declaration.name
            ));
        }
        // Check if the number of arguments matches
        if function.def.params.len() != declaration.args.len() {
            return Err(anyhow::anyhow!(
                "Function {} has {} parameters, but {} arguments were provided",
                declaration.name,
                function.def.params.len(),
                declaration.args.len()
            ));
        }
        // Check if the types of the arguments match
        for (i, arg) in declaration.args.iter().enumerate() {
            let arg_type = expression_check::check(arg, v_table, f_table, s_table)?;
            let param_type = &function.def.params[i].param_type;
            if arg_type != *param_type {
                return Err(anyhow::anyhow!(
                    "Type mismatch for argument {}: expected {:?}, found {:?}",
                    i + 1,
                    param_type,
                    arg_type
                ));
            }
        }
    } else {
        return Err(anyhow::anyhow!(
            "Function {} is not defined",
            declaration.name
        ));
    }

    Ok(())
}
