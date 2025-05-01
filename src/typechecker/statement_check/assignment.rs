use crate::ast::Expression;
use crate::typechecker::expression_check;
use crate::{
    ast::Assignment,
    typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable},
};
use anyhow::Result;

pub fn check(
    assignment: &Assignment,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
    debugger: &crate::debugger::Debugger,
    l: usize,
) -> Result<()> {
    let expression_value_type =
        expression_check::check(&assignment.value, v_table, f_table, s_table)?;
    let expression_target_type =
        expression_check::check(&assignment.target, v_table, f_table, s_table)?;

    if expression_value_type != expression_target_type {
        return Err(anyhow::anyhow!(
            "{}: Type mismatch: expected {:?}, found {:?}",
            debugger.display(l),
            expression_target_type,
            expression_value_type
        ));
    }

    if let Expression::Variable(ref var_name) = assignment.target {
        if let Some(var_type) = v_table.get(var_name) {
            if var_type.var_type != expression_value_type {
                return Err(anyhow::anyhow!(
                    "{}: Type mismatch: expected {:?}, found {:?}",
                    debugger.display(l),
                    var_type.var_type,
                    expression_value_type
                ));
            }
        } else {
            return Err(anyhow::anyhow!("Variable {} not declared", var_name));
        }
    }

    Ok(())
}
