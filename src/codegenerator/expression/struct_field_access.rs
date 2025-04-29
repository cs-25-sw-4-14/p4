use super::CodeGenerator;
use crate::ast::{StructFieldAccess, TopLevelDefinition, Expression};
use anyhow::Result;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct_field_access(
        &mut self,
        struct_field_access: &StructFieldAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
       
        let struct_val = self.compile_expression(&struct_field_access.struct_expr)?;
        let struct_ptr = match struct_val {
            BasicValueEnum::PointerValue(p) => p,
            _ => return Err(anyhow::anyhow!("Expected struct to be a pointer")),
        };

        
        let struct_type_name = match &*struct_field_access.struct_expr {
            Expression::Variable(var_name) => {
                match self.variables.get(var_name) {
                    Some((type_enum, _)) => self.get_struct_name(type_enum)
                        .ok_or_else(|| anyhow::anyhow!("Variable '{}' is not a struct", var_name))?,
                    None => return Err(anyhow::anyhow!("Variable '{}' not found", var_name)),
                }
            }
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

    fn get_struct_name(&self, type_enum: &BasicTypeEnum<'ctx>) -> Option<String> {
        if let BasicTypeEnum::StructType(struct_type) = type_enum {
            struct_type.get_name().map(|name| String::from_utf8_lossy(name.to_bytes()).into_owned())
        } else {
            None
        }
    }
}