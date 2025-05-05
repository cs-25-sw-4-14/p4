use crate::ast::{Read, Type}; // Assuming Read struct and Type enum are in `crate::ast`
use crate::typechecker::{
    expression_check, // Main checker to recursively check inner expressions
    function_table::FTable,
    struct_table::STable,
    variable_table::VTable,
};
use anyhow::{anyhow, Result}; // Using anyhow for error handling

/// Checks the `read_file(filename)` expression.
///
/// Ensures the `filename` argument evaluates to a `String`.
/// Returns `Type::String` as the result type of the expression.
pub fn check(
    read_expr: &Read, // Takes a reference to the Read struct from the AST
    v_table: &VTable, // Read-only access to variable types
    f_table: &FTable, // Read-only access to function signatures
    s_table: &STable, // Read-only access to struct definitions
) -> Result<Type> {
    // Check the type of the expression provided as the filename argument
    let filename_type = expression_check::check(&read_expr.value, v_table, f_table, s_table)?;

    // Validate that the filename expression evaluates to a String
    if filename_type != Type::String {
        // TODO: Enhance error reporting with Span/Location info if available from AST node
        return Err(anyhow!(
            "Type Error: `read_file` expects a String expression for the filename, but found type '{:?}'",
            filename_type
        ));
    }

    // If the filename type is correct, the `read_file` expression itself evaluates to a String (the file content)
    Ok(Type::String)
}