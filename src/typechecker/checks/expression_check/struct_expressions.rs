use std::collections::{HashMap, HashSet};
use crate::ast::{StructInit, StructFieldAccess, Type, Expression};
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use super::super::expression_check;
use super::type_utils::{types_match, can_convert_literal};
use anyhow::Result;

/// Type check a struct initialization expression
pub fn check_struct_init(
    struct_init: &StructInit,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<Type> {
    // Check if the struct exists
    let struct_def = match stable.get(&struct_init.name) {
        Some(def) => def,
        None => return Err(anyhow::anyhow!("Struct {} not found", struct_init.name)),
    };
    
    // Create a map of expected field types
    let mut field_types = HashMap::new();
    for field in &struct_def.fields {
        field_types.insert(&field.name, &field.field_type);
    }
    
    // Track which fields have been initialized
    let mut initialized_fields = HashSet::new();
    
    // Check each field initialization
    for field_init in &struct_init.fields {
        // Check for duplicate field initializations
        if !initialized_fields.insert(&field_init.name) {
            return Err(anyhow::anyhow!(
                "Duplicate initialization of field {} in struct {}",
                field_init.name, struct_init.name
            ));
        }
        
        // Check if the field exists in the struct
        let expected_type = match field_types.get(&field_init.name) {
            Some(field_type) => (*field_type).clone(),
            None => return Err(anyhow::anyhow!(
                "Field {} not found in struct {}", 
                field_init.name, struct_init.name
            )),
        };
        
        // Special handling for literal initializations
        if let Expression::Literal(literal) = &field_init.value {
            if can_convert_literal(literal, &expected_type) {
                continue; // Literal can be converted to the field type
            }
        }
        
        // Check if the field value has the expected type
        let actual_type = expression_check::check(&field_init.value, vtable, ftable, stable)?;
        
        // Compare types
        if !types_match(&actual_type, &expected_type) {
            return Err(anyhow::anyhow!(
                "Type mismatch for field {}: expected {:?}, got {:?}",
                field_init.name, expected_type, actual_type
            ));
        }
    }
    
    // Check if all fields are initialized
    if initialized_fields.len() != struct_def.fields.len() {
        // Find which fields were not initialized
        let missing_fields: Vec<_> = struct_def.fields.iter()
            .filter(|field| !initialized_fields.contains(&field.name))
            .map(|field| &field.name)
            .collect();
        
        return Err(anyhow::anyhow!(
            "Missing fields in struct {}: {:?}",
            struct_init.name, missing_fields
        ));
    }
    
    // All checks passed, return the struct type
    Ok(Type::Custom(struct_init.name.clone()))
}

/// Type check a struct field access expression
pub fn check_struct_field_access(
    struct_field_access: &StructFieldAccess,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<Type> {
    // Check the struct expression
    let struct_type = expression_check::check(&struct_field_access.struct_expr, vtable, ftable, stable)?;
    
    // Verify it's a struct type and find the field
    match struct_type {
        Type::Custom(struct_name) => {
            // Get the struct definition
            let struct_def = match stable.get(&struct_name) {
                Some(def) => def,
                None => return Err(anyhow::anyhow!("Struct {} not found", struct_name)),
            };
            
            // Look for the field
            for field in &struct_def.fields {
                if field.name == struct_field_access.field_name {
                    return Ok(field.field_type.clone());
                }
            }
            
            // Field not found
            Err(anyhow::anyhow!(
                "Field {} not found in struct {}", 
                struct_field_access.field_name, struct_name
            ))
        },
        _ => Err(anyhow::anyhow!(
            "Cannot access field {} on non-struct type {:?}",
            struct_field_access.field_name, struct_type
        ))
    }
}

