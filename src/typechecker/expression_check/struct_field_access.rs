use crate::ast::StructDef;
use crate::ast::{StructFieldAccess, Type};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_struct_field_access(
        &self,
        struct_field_access: &StructFieldAccess,
    ) -> Result<Type> {
        let struct_expr_type = self.check_expression(&struct_field_access.struct_expr)?;

        // Check if the expression type is a struct or a reference to a struct
        let struct_type = match struct_expr_type {
            Type::Custom(ref struct_name) => {
                self.semantic_check_struct_defined_for_name(struct_name)?
            }
            Type::Reference(ref_type, _) => {
                // Dereference the reference and check the struct definition for the referenced type
                if let Type::Custom(ref struct_name) = *ref_type {
                    self.semantic_check_struct_defined_for_name(&struct_name)?
                } else {
                    return Err(anyhow::anyhow!(
                        "struct field access must be on a struct or reference to a struct type"
                    ));
                }
            }
            _ => {
                return Err(anyhow::anyhow!(
                    "struct field access must be on a struct or reference to a struct type"
                ));
            }
        };

        // Now, check if the field exists in the struct
        let field_type = self.semantic_check_field_access_type_for_struct(
            &struct_type,
            &struct_field_access.field_name,
        )?;

        Ok(field_type)
    }

    fn semantic_check_struct_defined_for_name(&self, struct_name: &str) -> Result<&StructDef> {
        // Check if the struct is defined in the symbol table
        self.s_table
            .structs
            .get(struct_name)
            .ok_or_else(|| anyhow::anyhow!("struct: {}, not defined", struct_name))
    }

    fn semantic_check_field_access_type_for_struct(
        &self,
        struct_definition: &StructDef,
        field_name: &str,
    ) -> Result<Type> {
        // Check if the field is defined in the struct
        struct_definition
            .fields
            .iter()
            .find(|field| field.name == field_name)
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "field: {}, not defined in struct: {}",
                    field_name,
                    struct_definition.name
                )
            })
            .map(|field| field.field_type.clone())
    }
}
