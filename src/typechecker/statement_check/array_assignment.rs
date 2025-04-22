use crate::ast::Type;
use crate::{
    ast::ArrayAssignment,
    typechecker::expression_check,
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    array_assignment: &ArrayAssignment,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<()> {
    // Check if the array is declared
    let array = v_table
        .get(&array_assignment.target.array_name)
        .ok_or_else(|| {
            anyhow::anyhow!("Array {} not declared", array_assignment.target.array_name)
        })?;

    // check value type in array assignment
    let value_type = expression_check::check(&array_assignment.value, v_table, f_table, s_table)?;

    match &array.var_type {
        Type::Array(ty, _) => {
            if **ty != value_type {
                return Err(anyhow::anyhow!(
                    "Type mismatch: expected {:?}, found {:?}",
                    array.var_type,
                    value_type
                ));
            }
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Array assignment on non-array type: {:?}",
                array.var_type
            ));
        }
    }

    Ok(())
}
