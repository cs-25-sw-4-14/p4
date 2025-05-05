use crate::ast::{Write, Type}; // Assuming Write struct and Type enum are in `crate::ast`
use crate::typechecker::{
    expression_check, // Main checker to recursively check inner expressions
    function_table::FTable,
    struct_table::STable,
    variable_table::VTable,
};
use anyhow::{anyhow, Result}; // Using anyhow for error handling

/// Checks the `write_file(value, target)` expression.
///
/// Ensures both the `value` (content) and `target` (filename) arguments
/// evaluate to `Type::String`.
/// Returns `Type::Void` as the result type of the expression, as `write_file`
/// performs an action but doesn't produce a value itself.
pub fn check(
    write_expr: &Write, // Takes a reference to the Write struct from the AST
    v_table: &VTable,  // Read-only access to variable types
    f_table: &FTable,  // Read-only access to function signatures
    s_table: &STable,  // Read-only access to struct definitions
) -> Result<Type> {
    // 1. Check the type of the 'value' expression (the content to write)
    let value_type = expression_check::check(&write_expr.value, v_table, f_table, s_table)?;

    // Validate that the value expression evaluates to a String
    if value_type != Type::String {
        // TODO: Enhance error reporting with Span/Location info if available from AST node
        return Err(anyhow!(
            "Type Error: `write_file` expects a String expression for the 'value' argument (content), but found type '{:?}'",
            value_type
        ));
    }

    // 2. Check the type of the 'target' expression (the filename)
    let target_type = expression_check::check(&write_expr.target, v_table, f_table, s_table)?;

    // Validate that the target expression evaluates to a String
    if target_type != Type::String {
        // TODO: Enhance error reporting with Span/Location info if available from AST node
        return Err(anyhow!(
            "Type Error: `write_file` expects a String expression for the 'target' argument (filename), but found type '{:?}'",
            target_type
        ));
    }

    // If both arguments are correct, the `write_file` expression itself performs an action
    // and doesn't return a value, so its type is Void.
    // Ensure `Type::Void` is defined in your `crate::ast::Type` enum.
    Ok(Type::Void)
}