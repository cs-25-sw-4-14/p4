use super::CodeGenerator;
use crate::ast::{Expression, StructFieldAccess};
use anyhow::{Result, anyhow};
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct_field_access(
        &mut self,
        struct_field_access: &StructFieldAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        let struct_name = if let Expression::Variable(var_name) = &*struct_field_access.struct_expr
        {
            var_name
        } else {
            return Err(anyhow!("Struct field access must be on a variable"));
        };

        let (struct_type, struct_ptr) = self
            .variables
            .get(struct_name)
            .ok_or_else(|| anyhow!("Struct variable '{}' not found", struct_name))?;

        let struct_type_name = if let BasicTypeEnum::StructType(struct_ty) = struct_type {
            if let Some(name) = struct_ty.get_name() {
                name.to_str().map_err(|_| {
                    anyhow!(
                        "Struct name for variable '{}' is not valid UTF-8",
                        struct_name
                    )
                })?
            } else {
                return Err(anyhow!(
                    "Struct type for variable '{}' has no name",
                    struct_name
                ));
            }
        } else {
            return Err(anyhow!(
                "Expected a struct type for variable '{}'",
                struct_name
            ));
        };

        let field_index =
            self.get_field_index(struct_type_name, &struct_field_access.field_name)?;

        let struct_def = self
            .struct_definitions
            .get(struct_type_name)
            .ok_or_else(|| anyhow!("Struct definition '{}' not found", struct_type_name))?;

        let field_ptr = self.builder.build_struct_gep(
            *struct_type,
            *struct_ptr,
            field_index as u32,
            &format!(
                "{}_field_{}",
                struct_type_name, struct_field_access.field_name
            ),
        )?;

        let field_type = self.get_llvm_type(&struct_def.fields[field_index].field_type)?;

        let field_value = self.builder.build_load(
            field_type,
            field_ptr,
            &format!(
                "{}_field_value_{}",
                struct_type_name, struct_field_access.field_name
            ),
        )?;

        Ok(field_value)
    }
}
