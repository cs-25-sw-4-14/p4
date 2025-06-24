use super::CodeGenerator;
use crate::ast::StructFieldAccess;
use anyhow::{Result, anyhow};
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles access to a field of a struct variable.
    ///
    /// # Arguments
    /// * `struct_field_access` - The AST node representing the field access (e.g., `myStruct.field`).
    ///
    /// # Returns
    /// Returns the LLVM IR value representing the loaded field value.
    ///
    /// # Errors
    /// Returns an error if:
    /// - The struct variable is not found,
    /// - The variable is not a struct type,
    /// - The struct type has no valid name,
    /// - The struct definition or field is not found.
    pub fn compile_struct_field_access(
        &mut self,
        struct_field_access: &StructFieldAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Get the struct variable name as a string slice
        let struct_name = &*struct_field_access.struct_name;

        // Look up the variable (struct) in the current symbol table
        // `variables` stores mapping from variable names to (type, pointer)
        let (struct_type, struct_ptr) = self
            .variables
            .get(struct_name)
            .ok_or_else(|| anyhow!("Struct variable '{}' not found", struct_name))?;

        // Ensure that the variable type is indeed a struct type and get the struct type name
        let struct_type_name = if let BasicTypeEnum::StructType(struct_ty) = struct_type {
            // Get the name of the struct type, ensuring it is valid UTF-8
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
            // If not a struct type, return an error
            return Err(anyhow!(
                "Expected a struct type for variable '{}'",
                struct_name
            ));
        };

        // Get the field index for the field being accessed within the struct definition
        let field_index =
            self.get_field_index(struct_type_name, &struct_field_access.field_name)?;

        // Retrieve the full struct definition (AST or metadata) to get field info
        let struct_def = self
            .struct_definitions
            .get(struct_type_name)
            .ok_or_else(|| anyhow!("Struct definition '{}' not found", struct_type_name))?;

        // Generate LLVM IR code to get a pointer to the specific struct field using GEP
        // GEP (GetElementPointer) computes the address of the field inside the struct
        let field_ptr = self.builder.build_struct_gep(
            *struct_type,
            *struct_ptr,
            field_index as u32,
            &format!(
                "{}_field_{}",
                struct_type_name, struct_field_access.field_name
            ),
        )?;

        // Determine the LLVM type of the field (needed for loading it)
        let field_type = self.get_llvm_type(&struct_def.fields[field_index].field_type)?;

        // Load the value from the field pointer (read the field value)
        let field_value = self.builder.build_load(
            field_type,
            field_ptr,
            &format!(
                "{}_field_value_{}",
                struct_type_name, struct_field_access.field_name
            ),
        )?;

        // Return the loaded field value to the caller
        Ok(field_value)
    }
}
