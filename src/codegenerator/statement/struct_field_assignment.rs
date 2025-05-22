use crate::{ast::StructFieldAssignment, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::types::BasicTypeEnum;

impl CodeGenerator<'_> {
    pub fn compile_struct_field_assignment(
        &mut self,
        assign: &StructFieldAssignment,
    ) -> Result<()> {
        let new_value = self.compile_expression(&assign.value)?;
        let struct_name = assign.target.clone();
        let field_name = assign.field_name.clone();

        let (struct_type, struct_ptr) = self
            .variables
            .get(&struct_name)
            .ok_or_else(|| anyhow!("Struct variable '{:?}' not found", struct_name))?;

        let struct_type = if let BasicTypeEnum::StructType(ty) = struct_type {
            *ty
        } else {
            return Err(anyhow!(
                "Expected struct type for variable '{}'",
                struct_name
            ));
        };

        let field_index =
            self.get_field_index(struct_type.get_name().unwrap().to_str()?, &field_name)?;

        let field_type = struct_type;

        let field_ptr = self.builder.build_struct_gep(
            struct_type,
            *struct_ptr,
            field_index as u32,
            &format!("{}_field_ptr_{}", struct_name, field_name),
        )?;

        self.builder
            .build_store(field_ptr, new_value)
            .map_err(|e| {
                anyhow!(
                    "Failed to store new value in struct field '{}': {}",
                    field_name,
                    e
                )
            })?;

        self.last_value = Some(new_value);
        self.builder.build_load(
            field_type,
            field_ptr,
            &format!("{}_field_val_{}", struct_name, field_name),
        )?;

        Ok(())
    }
}
