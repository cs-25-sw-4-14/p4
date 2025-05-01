use super::CodeGenerator;
use crate::ast::{StructDef, StructInit, TopLevelDefinition};
use anyhow::Result;
use inkwell::values::BasicValueEnum;
use inkwell::types::BasicType;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct_init(
        &mut self,
        struct_init: &StructInit,
    ) -> Result<BasicValueEnum<'ctx>> {
        let struct_name = &struct_init.name;
        let struct_type = self
            .get_struct_type(struct_name)?
            .ok_or_else(|| anyhow::anyhow!("Struct type '{}' not found", struct_name))?;

        let struct_ptr = self
            .builder
            .build_alloca(struct_type, &format!("{}_instance", struct_name))?;

        // Process each field initialization
        for field_init in &struct_init.fields {
            // Get the field index using its name
            let field_index = self.get_field_index(struct_name, &field_init.name)?;
            
            // Compile the field value
            let field_value = self.compile_expression(&field_init.value)?;
            
            // Create GEP to get pointer to the field
            let field_ptr = {
                self.builder.build_struct_gep(
                    struct_type,
                    struct_ptr,
                    field_index as u32,
                    &format!("{}_field_{}", struct_name, field_init.name),
                )?
            };
            
            // Store the value in the field
            self.builder.build_store(field_ptr, field_value)?;
        }

        // Register the struct as a heap allocation for later deallocation
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.to_free.push(super::super::ScopedVariable {
                name: format!("{}_instance", struct_name),
                var_type: struct_type.as_any_type_enum().try_into().unwrap(),
                ptr: struct_ptr,
                needs_free: true, // Structs are heap-allocated in this implementation
            });
        }
        
        let result = BasicValueEnum::PointerValue(struct_ptr);
        self.last_value = Some(result);
        Ok(result)
    }

    fn get_struct_type(
        &self,
        struct_name: &str,
    ) -> Result<Option<inkwell::types::StructType<'ctx>>> {
        Ok(self.module.get_struct_type(struct_name))
    }

    fn get_field_index(&self, struct_name: &str, field_name: &str) -> Result<usize> {
        let struct_def = self.find_struct_definition(struct_name)?;
        struct_def
            .fields
            .iter()
            .position(|field| field.name == field_name)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Field '{}' not found in struct '{}'",
                    field_name,
                    struct_name
                )
            })
    }

    fn find_struct_definition(&self, struct_name: &str) -> Result<&StructDef> {
        match &self.program {
            Some(program) => program
                .definitions
                .iter()
                .filter_map(|def| match def {
                    TopLevelDefinition::Struct(s) => Some(s),
                    _ => None,
                })
                .find(|s| s.name == struct_name)
                .ok_or_else(|| anyhow::anyhow!("Struct definition '{}' not found", struct_name)),
            None => Err(anyhow::anyhow!("No program loaded")),
        }
    }
}
