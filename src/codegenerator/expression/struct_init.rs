use super::CodeGenerator;
use crate::ast::{StructDef, StructInit};
use anyhow::Result;
use inkwell::values::BasicValueEnum;

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

        for field_init in &struct_init.fields {
            let field_index = self.get_field_index(struct_name, &field_init.name)?;
            let field_ptr = self.builder.build_struct_gep(
                struct_type,
                struct_ptr,
                field_index as u32,
                &format!("{}_field_ptr_{}", struct_name, field_init.name),
            )?;
            let field_value = self.compile_expression(&field_init.value)?;
            self.builder.build_store(field_ptr, field_value)?;
        }

        let struct_value =
            self.builder
                .build_load(struct_type, struct_ptr, &format!("{}_value", struct_name))?;

        Ok(struct_value)
    }

    pub fn get_struct_type(
        &self,
        struct_name: &str,
    ) -> Result<Option<inkwell::types::StructType<'ctx>>> {
        Ok(self.module.get_struct_type(struct_name))
    }

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

    pub fn find_struct_definition(&self, struct_name: &str) -> Result<&StructDef> {
        self.struct_definitions
            .get(struct_name)
            .ok_or_else(|| anyhow::anyhow!("Struct definition '{}' not found", struct_name))
    }
}
