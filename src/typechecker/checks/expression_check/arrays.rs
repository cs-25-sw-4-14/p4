use crate::ast::{ArrayAccess, ArrayLiteral, Type, Expression, Literal};
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use super::super::expression_check;
use super::type_utils::{types_match, is_integer_type};
use anyhow::Result;

/// Type check an array access expression
pub fn check_array_access(
    array_access: &ArrayAccess,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<Type> {
    // Look up the array variable
    let array_type = if let Some(var) = vtable.get(&array_access.array_name) {
        var.var_type.clone()
    } else {
        return Err(anyhow::anyhow!("Array variable {} not found", array_access.array_name));
    };
    
    // Check if it's actually an array or vector type
    match array_type {
        Type::Array(element_type, size) => {
            // Check that the index expression is an integer
            let index_type = expression_check::check(&array_access.index, vtable, ftable, stable)?;
            
            // Perform bounds checking if index is a literal integer
            if let Expression::Literal(Literal::Int(index)) = &array_access.index {
                if *index < 0 || *index >= size {
                    return Err(anyhow::anyhow!(
                        "Array index out of bounds: index {} for array of size {}", 
                        index, size
                    ));
                }
            }
            
            if is_integer_type(&index_type) {
                // Return the element type
                Ok(*element_type)
            } else {
                Err(anyhow::anyhow!(
                    "Array index must be an integer, got {:?}", index_type
                ))
            }
        },
        Type::Vector(element_type) => {
            // For vectors, we can't check bounds statically since their size is dynamic
            // We can only check that the index type is correct
            let index_type = expression_check::check(&array_access.index, vtable, ftable, stable)?;
            if is_integer_type(&index_type) {
                // Return the element type
                Ok(*element_type)
            } else {
                Err(anyhow::anyhow!(
                    "Vector index must be an integer, got {:?}", index_type
                ))
            }
        },
        _ => Err(anyhow::anyhow!(
            "Cannot index into non-array type {:?}", array_type
        )),
    }
}

/// Type check an array literal expression
pub fn check_array_literal(
    array_literal: &ArrayLiteral,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<Type> {
    if array_literal.elements.is_empty() {
        // Empty array - we can't determine the element type
        // For now, return a placeholder type
        return Err(anyhow::anyhow!("Empty array literals are not supported"));
    }
    
    // Check the type of the first element
    let first_element_type = expression_check::check(&array_literal.elements[0], vtable, ftable, stable)?;
    
    // Check that all elements have the same type
    for (i, element) in array_literal.elements.iter().enumerate().skip(1) {
        let element_type = expression_check::check(element, vtable, ftable, stable)?;
        if !types_match(&element_type, &first_element_type) {
            return Err(anyhow::anyhow!(
                "Array elements must have the same type: element 0 has type {:?}, but element {} has type {:?}",
                first_element_type, i, element_type
            ));
        }
    }
    
    // Return an array type with the element type
    Ok(Type::Array(Box::new(first_element_type), array_literal.elements.len() as i64))
}


