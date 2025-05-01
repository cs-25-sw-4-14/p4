use super::CodeGenerator;
use crate::ast::{StructFieldAccess, TopLevelDefinition, Expression};
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct_field_access(
        &mut self,
        struct_field_access: &StructFieldAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Handle different ways to get a struct pointer based on the expression type
        let struct_ptr = match &*struct_field_access.struct_expr {
            Expression::Variable(var_name) => {
                // For variables, we can just get the existing allocated pointer
                let (_, ptr) = self.variables.get(var_name)
                    .copied()
                    .ok_or_else(|| anyhow::anyhow!("Variable '{}' not found", var_name))?;
                ptr
            },
            Expression::StructFieldAccess(_) => {
                // For nested field access, we need to fully evaluate the expression
                let struct_val = self.compile_expression(&struct_field_access.struct_expr)?;
                match struct_val {
                    BasicValueEnum::PointerValue(p) => p,
                    _ => return Err(anyhow::anyhow!("Expected struct to be a pointer (nested field)")),
                }
            },
            _ => {
                // For other expressions, try to evaluate to a pointer
                let struct_val = self.compile_expression(&struct_field_access.struct_expr)?;
                match struct_val {
                    BasicValueEnum::PointerValue(p) => p,
                    _ => return Err(anyhow::anyhow!("Expected struct to be a pointer (expression)")),
                }
            }
        };

        
        let struct_type_name = match &*struct_field_access.struct_expr {
            Expression::Variable(var_name) => {
                match self.variables.get(var_name) {
                    Some((type_enum, _)) => self.get_struct_name(type_enum)
                        .ok_or_else(|| anyhow::anyhow!("Variable '{}' is not a struct", var_name))?,
                    None => return Err(anyhow::anyhow!("Variable '{}' not found", var_name)),
                }
            }
            Expression::StructFieldAccess(nested_access) => {
                // For now, let's implement a simpler approach to handle the immediate use case
                // Get the variable name and field path
                if let Expression::Variable(var_name) = &*nested_access.struct_expr {
                    // Get the struct variable
                    let (struct_type, _) = self.variables.get(var_name)
                        .copied()
                        .ok_or_else(|| anyhow::anyhow!("Variable '{}' not found", var_name))?;
                        
                    // Get struct type name
                    let parent_struct_type = self.get_struct_name(&struct_type)
                        .ok_or_else(|| anyhow::anyhow!("Variable '{}' is not a struct", var_name))?;
                    
                    // Find field's type in the struct definition
                    let field_type = self.program.as_ref()
                        .ok_or_else(|| anyhow::anyhow!("Program not initialized"))?
                        .definitions.iter()
                        .find_map(|def| {
                            if let TopLevelDefinition::Struct(def) = def {
                                if def.name == parent_struct_type {
                                    def.fields.iter()
                                        .find(|f| f.name == nested_access.field_name)
                                        .map(|f| &f.field_type)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .ok_or_else(|| anyhow::anyhow!("Field '{}' not found in struct '{}'", 
                            nested_access.field_name, parent_struct_type))?;
                    
                    // Check if the field is a struct
                    if let crate::ast::Type::Struct(struct_name) = field_type {
                        struct_name.clone()
                    } else {
                        return Err(anyhow::anyhow!("Nested field '{}' is not a struct type", nested_access.field_name));
                    }
                } else {
                    return Err(anyhow::anyhow!("Unsupported nested struct expression"));
                }
            },
            _ => return Err(anyhow::anyhow!("Unsupported struct expression")),
        };

        
        let field_index = self.program.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Program not initialized"))?
            .definitions.iter()
            .filter_map(|def| {
                if let TopLevelDefinition::Struct(def) = def {
                    (def.name == struct_type_name).then_some(def)
                } else {
                    None
                }
            })
            .next()
            .ok_or_else(|| anyhow::anyhow!("Struct '{}' not found", struct_type_name))?
            .fields.iter()
            .position(|f| f.name == struct_field_access.field_name)
            .ok_or_else(|| anyhow::anyhow!("Field '{}' not found", struct_field_access.field_name))?;

        // Get the struct type from the pointed struct
        let struct_type = self.module.get_struct_type(&struct_type_name)
            .ok_or_else(|| anyhow::anyhow!("Struct type '{}' not found in module", struct_type_name))?;
       
        // Build struct GEP with the correct struct type
        let field_ptr = self.builder.build_struct_gep(
            struct_type,
            struct_ptr,
            field_index as u32,
            "field_ptr",
        )?;

        
        let field_ast_type = self.program.as_ref()
            .unwrap()
            .definitions.iter()
            .filter_map(|def| {
                if let TopLevelDefinition::Struct(def) = def {
                    (def.name == struct_type_name).then_some(&def.fields[field_index].field_type)
                } else {
                    None
                }
            })
            .next()
            .ok_or_else(|| anyhow::anyhow!("Could not determine field type"))?;

        let llvm_field_type = self.get_llvm_type(field_ast_type)?;
        
        let field_value = self.builder.build_load(llvm_field_type, field_ptr ,"field_value")?;

        Ok(field_value)
    }

    // Function moved to CodeGenerator impl
}