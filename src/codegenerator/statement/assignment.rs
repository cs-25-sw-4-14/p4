use crate::{ast::VariableAssignment, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_variable_assignment(&mut self, assign: &VariableAssignment) -> Result<()> {
        let new_value = self.compile_expression(&assign.value)?;
        let variable_name = &assign.target;

        // Check if this is a struct field access (contains dot notation)
        if variable_name.contains('.') {
            let parts: Vec<&str> = variable_name.split('.').collect();
            
            // Get the root struct variable
            let root_struct_name = parts[0];
            let (struct_type, struct_ptr) = self.variables.get(root_struct_name)
                .copied()
                .ok_or_else(|| anyhow!("Variable '{}' not found", root_struct_name))?;
            
            // Get the struct type name
            let mut current_struct_type_name = self.get_struct_name(&struct_type)
                .ok_or_else(|| anyhow!("Variable '{}' is not a struct", root_struct_name))?;
                
            let mut current_struct_type = self.module.get_struct_type(&current_struct_type_name)
                .ok_or_else(|| anyhow!("Struct type '{}' not found in module", current_struct_type_name))?;
                
            let mut current_ptr = struct_ptr;
            
            // For each field in the access chain except the last one
            for i in 1..parts.len()-1 {
                let field_name = parts[i];
                
                // Get the field index
                let field_index = self.program.as_ref()
                    .ok_or_else(|| anyhow!("Program not initialized"))?
                    .definitions.iter()
                    .filter_map(|def| {
                        if let crate::ast::TopLevelDefinition::Struct(def) = def {
                            (def.name == current_struct_type_name).then_some(def)
                        } else {
                            None
                        }
                    })
                    .next()
                    .ok_or_else(|| anyhow!("Struct '{}' not found", current_struct_type_name))?
                    .fields.iter()
                    .position(|f| f.name == field_name)
                    .ok_or_else(|| anyhow!("Field '{}' not found in struct '{}'", field_name, current_struct_type_name))?;
                
                // Get a pointer to the intermediate field
                let field_ptr = self.builder.build_struct_gep(
                    current_struct_type,
                    current_ptr,
                    field_index as u32,
                    &format!("intermediate_field_ptr")
                )?;
                
                // Update current pointer to the field
                current_ptr = field_ptr;
                
                // Get the type of this field, which should be another struct
                let field_type = &self.program.as_ref().unwrap()
                    .definitions.iter()
                    .filter_map(|def| {
                        if let crate::ast::TopLevelDefinition::Struct(def) = def {
                            (def.name == current_struct_type_name).then_some(def)
                        } else {
                            None
                        }
                    })
                    .next()
                    .unwrap()
                    .fields[field_index].field_type;
                
                // Update current struct type for the next iteration
                if let crate::ast::Type::Struct(name) = field_type {
                    current_struct_type_name = name.clone();
                    current_struct_type = self.module.get_struct_type(&current_struct_type_name)
                        .ok_or_else(|| anyhow!("Struct type '{}' not found in module", current_struct_type_name))?;
                } else {
                    return Err(anyhow!("Field '{}' is not a struct", field_name));
                }
            }
            
            // Now process the final field in the chain
            let final_field_name = parts[parts.len()-1];
            
            // Get the field index in the current struct
            let field_index = self.program.as_ref()
                .unwrap()
                .definitions.iter()
                .filter_map(|def| {
                    if let crate::ast::TopLevelDefinition::Struct(def) = def {
                        (def.name == current_struct_type_name).then_some(def)
                    } else {
                        None
                    }
                })
                .next()
                .ok_or_else(|| anyhow!("Struct '{}' not found", current_struct_type_name))?
                .fields.iter()
                .position(|f| f.name == final_field_name)
                .ok_or_else(|| anyhow!("Field '{}' not found in struct '{}'", final_field_name, current_struct_type_name))?;
                
            // Get a pointer to the final field
            let field_ptr = self.builder.build_struct_gep(
                current_struct_type,
                current_ptr,
                field_index as u32,
                &format!("final_field_ptr")
            )?;
            
            // Store the new value in the field
            self.builder.build_store(field_ptr, new_value)?;
            
            Ok(())
        } else {
            // Regular variable assignment
            let ptr = self.variables.get(variable_name)
                .copied() // Get the pointer value for the variable from the symbol table
                .ok_or_else(|| {
                    anyhow!("Compiler Bug: Variable '{}' not found in symbol table during assignment compilation.", variable_name)
                })?;

            self.builder
                .build_store(ptr.1, new_value)
                .map_err(|e| anyhow!("Failed to store new value in '{}': {}", variable_name, e))?;

            Ok(())
        }
    }
}
