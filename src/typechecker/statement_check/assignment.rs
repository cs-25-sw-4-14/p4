use crate::ast::{Expression, Type};
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
        // Already handled
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
    } else if let Expression::StructFieldAccess(ref access) = assignment.target {
        // Get type of struct expression
        let struct_type = expression_check::check(&access.struct_expr, v_table, f_table, s_table)?;
        if let Type::Struct(ref struct_name) = struct_type {
            if let Some(struct_def) = s_table.get(struct_name) {
                if let Some(field) = struct_def
                    .fields
                    .iter()
                    .find(|f| f.name == access.field_name)
                {
                    if field.field_type != expression_value_type {
                        return Err(anyhow::anyhow!(
                            "{}: Type mismatch: expected {:?}, found {:?}",
                            debugger.display(l),
                            field.field_type,
                            expression_value_type
                        ));
                    }
                } else {
                    return Err(anyhow::anyhow!(
                        "{}: Field {} not found in struct {}",
                        debugger.display(l),
                        access.field_name,
                        struct_name
                    ));
                }
            } else {
                return Err(anyhow::anyhow!(
                    "{}: Struct {} not found",
                    debugger.display(l),
                    struct_name
                ));
            }
        } else {
            return Err(anyhow::anyhow!(
                "{}: Attempted to access field on non-struct type {:?}",
                debugger.display(l),
                struct_type
            ));
        }
    } else if let Expression::ArrayAccess(ref access) = assignment.target {
        // Look up the array variable
        if let Some(var_type) = v_table.get(&access.array_name) {
            if let Type::Array(boxed_element_type, _) = &var_type.var_type {
                if **boxed_element_type != expression_value_type {
                    return Err(anyhow::anyhow!(
                        "{}: Type mismatch: expected {:?}, found {:?}",
                        debugger.display(l),
                        boxed_element_type,
                        expression_value_type
                    ));
                }
            } else {
                return Err(anyhow::anyhow!(
                    "{}: Variable {} is not an array",
                    debugger.display(l),
                    access.array_name
                ));
            }
        } else {
            return Err(anyhow::anyhow!(
                "{}: Array variable {} not declared",
                debugger.display(l),
                access.array_name
            ));
        }
    }

    Ok(())
}
