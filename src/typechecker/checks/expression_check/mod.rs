use crate::ast::{Expression, Type};
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use anyhow::Result;

// Import submodules
mod type_utils;
mod literals;
mod binary_ops;
mod unary_ops;
mod function_calls;
mod struct_expressions;
mod arrays;
pub mod utils;

// Re-export utility functions
pub use type_utils::{types_match, can_convert_literal, is_numeric_type, is_integer_type};

/// Main entry point for expression type checking
pub fn check(
    expression: &Expression, 
    vtable: &VTable, 
    ftable: &Ftable,
    stable: &STable
) -> Result<Type> {
    match expression {
        Expression::Literal(literal) => 
            literals::check_literal(literal),
            
        Expression::Variable(name) => 
            check_variable(name, vtable),
            
        Expression::BinaryOp(binary_operation) => 
            binary_ops::check_binary_op(binary_operation, vtable, ftable, stable),
            
        Expression::UnaryOp(unary_operation) => 
            unary_ops::check_unary_op(unary_operation, vtable, ftable, stable),
            
        Expression::FunctionCall(function_call) => 
            function_calls::check_function_call(function_call, vtable, ftable, stable),
            
        Expression::StructInit(struct_init) => 
            struct_expressions::check_struct_init(struct_init, vtable, ftable, stable),
            
        Expression::StructFieldAccess(struct_field_access) => 
            struct_expressions::check_struct_field_access(struct_field_access, vtable, ftable, stable),
            
        Expression::ArrayAccess(array_access) => 
            arrays::check_array_access(array_access, vtable, ftable, stable),
            
        Expression::ArrayLiteral(array_literal) => 
            arrays::check_array_literal(array_literal, vtable, ftable, stable),
            
        Expression::TypeCast(type_cast) => 
            check_type_cast(type_cast, vtable, ftable, stable),
    }
}

// Helper function for variable lookup
fn check_variable(name: &str, vtable: &VTable) -> Result<Type> {
    if let Some(var) = vtable.get(name) {
        Ok(var.var_type.clone())
    } else {
        Err(anyhow::anyhow!("Variable {} not found in variable table", name))
    }
}

// Helper function for type cast checking
fn check_type_cast(
    type_cast: &crate::ast::TypeCast,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<Type> {
    // Check the expression being cast
    let source_type = check(&type_cast.expression, vtable, ftable, stable)?;
    let target_type = type_cast.target_type.clone();
    
    // Check if the cast is valid
    if type_utils::is_valid_cast(&source_type, &target_type) {
        Ok(target_type)
    } else {
        Err(anyhow::anyhow!(
            "Invalid cast from {:?} to {:?}", source_type, target_type
        ))
    }
}
