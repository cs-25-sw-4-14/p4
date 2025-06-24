use super::CodeGenerator;
use crate::ast::{StructDef, StructInit};
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a struct instantiation like: `MyStruct { field1: value1, field2: value2 }`
    pub fn compile_struct_init(
        &mut self,
        struct_init: &StructInit,
    ) -> Result<BasicValueEnum<'ctx>> {
        let struct_name = &struct_init.name;

        // Get the LLVM struct type from the module by name
        let struct_type = self
            .get_struct_type(struct_name)?
            .ok_or_else(|| anyhow::anyhow!("Struct type '{}' not found", struct_name))?;

        // Allocate memory for the struct instance on the stack
        let struct_ptr = self
            .builder
            .build_alloca(struct_type, &format!("{}_instance", struct_name))?;

        // Loop through each field initializer and store its value in the corresponding slot
        for field_init in &struct_init.fields {
            // Get the index of the field within the struct type
            let field_index = self.get_field_index(struct_name, &field_init.name)?;

            // Get pointer to the specific field using GEP
            let field_ptr = self.builder.build_struct_gep(
                struct_type,
                struct_ptr,
                field_index as u32,
                &format!("{}_field_ptr_{}", struct_name, field_init.name),
            )?;

            // Compile the initializer expression for the field
            let field_value = self.compile_expression(&field_init.value)?;

            // Store the value into the struct field
            self.builder.build_store(field_ptr, field_value)?;
        }

        // Load the initialized struct into a value (this makes it a first-class value)
        let struct_value =
            self.builder
                .build_load(struct_type, struct_ptr, &format!("{}_value", struct_name))?;

        Ok(struct_value)
    }

    /// Gets the LLVM StructType from the module by name
    pub fn get_struct_type(
        &self,
        struct_name: &str,
    ) -> Result<Option<inkwell::types::StructType<'ctx>>> {
        Ok(self.module.get_struct_type(struct_name))
    }

    /// Gets the index of a field in a struct by name
    pub fn get_field_index(&self, struct_name: &str, field_name: &str) -> Result<usize> {
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

    /// Gets the original struct definition (likely from the parsed AST)
    pub fn find_struct_definition(&self, struct_name: &str) -> Result<&StructDef> {
        self.struct_definitions
            .get(struct_name)
            .ok_or_else(|| anyhow::anyhow!("Struct definition '{}' not found", struct_name))
    }
}
