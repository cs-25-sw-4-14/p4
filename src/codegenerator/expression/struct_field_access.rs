use super::CodeGenerator;
use crate::ast::{Expression, StructFieldAccess};
use anyhow::Result;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct_field_access(
        &mut self,
        struct_field_access: &StructFieldAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        let struct_val = self.compile_expression(&struct_field_access.struct_expr)?;

        let struct_type_name = match &*struct_field_access.struct_expr {
            Expression::Variable(var_name) => match self.variables.get(var_name) {
                Some((type_enum, _)) => {
                    if let BasicTypeEnum::StructType(struct_type) = type_enum {
                        struct_type
                            .get_name()
                            .ok_or_else(|| anyhow::anyhow!("Struct type has no name"))?
                            .to_str()
                            .map_err(|_| anyhow::anyhow!("Invalid UTF-8 in struct type name"))?
                            .to_string()
                    } else {
                        return Err(anyhow::anyhow!("Variable '{}' is not a struct", var_name));
                    }
                }
                None => return Err(anyhow::anyhow!("Variable '{}' not found", var_name)),
            },
            _ => return Err(anyhow::anyhow!("Unsupported struct expression")),
        };

        let field_index = {
            let struct_def = self.find_struct_definition(&struct_type_name)?;
            struct_def
                .fields
                .iter()
                .position(|f| f.name == struct_field_access.field_name)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Field '{}' not found in struct '{}'",
                        struct_field_access.field_name,
                        struct_type_name
                    )
                })?
        };

        let struct_type = self.get_struct_type(&struct_type_name)?.ok_or_else(|| {
            anyhow::anyhow!("LLVM type for struct '{}' not found", struct_type_name)
        })?;

        let struct_ptr = match struct_val {
            BasicValueEnum::PointerValue(p) => p,
            _ => {
                let alloca = self
                    .builder
                    .build_alloca(struct_val.get_type(), "struct_temp")?;
                self.builder.build_store(alloca, struct_val)?;
                alloca
            }
        };

        let field_ptr = self.builder.build_struct_gep(
            struct_type,
            struct_ptr,
            field_index as u32,
            &format!(
                "{}_field_{}",
                struct_type_name, struct_field_access.field_name
            ),
        )?;

        let field_type = struct_type.get_field_types()[field_index];
        let field_val = self
            .builder
            .build_load(field_type, field_ptr, "load_field")?;

        println!(
            "Struct field access: {}.{} -> {:?}",
            struct_type_name, struct_field_access.field_name, field_val
        );

        Ok(field_val)
    }
}
