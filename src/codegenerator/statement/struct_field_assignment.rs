use crate::{ast::StructFieldAssignment, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::types::BasicTypeEnum;

impl CodeGenerator<'_> {
    /// Compiles an assignment to a field inside a struct.
    ///
    /// Example: `myStruct.fieldName = newValue;`
    pub fn compile_struct_field_assignment(
        &mut self,
        assign: &StructFieldAssignment,
    ) -> Result<()> {
        // Compile the expression for the new value that will be stored in the struct field
        let new_value = self.compile_expression(&assign.value)?;

        // Extract the target struct variable's name and the field name from the assignment
        let struct_name = assign.target.clone();
        let field_name = assign.field_name.clone();

        // Look up the struct variable in the symbol table (variables map)
        // This returns a tuple: (struct type, pointer to the struct)
        let (struct_type, struct_ptr) = self
            .variables
            .get(&struct_name)
            .ok_or_else(|| anyhow!("Struct variable '{:?}' not found", struct_name))?;

        // Ensure that the variable is actually a struct type
        // If not, return an error
        let struct_type = if let BasicTypeEnum::StructType(ty) = struct_type {
            *ty
        } else {
            return Err(anyhow!(
                "Expected struct type for variable '{}'",
                struct_name
            ));
        };

        // Find the index of the field within the struct type by name
        // This is necessary to access the correct field position
        let field_index =
            self.get_field_index(struct_type.get_name().unwrap().to_str()?, &field_name)?;

        // The field_type is the struct_type itself here, used for loading later
        // (You might want to get the specific field type if needed)
        let field_type = struct_type;

        // Use LLVM’s `build_struct_gep` to get a pointer to the specific struct field
        // `field_index` specifies the offset within the struct
        let field_ptr = self.builder.build_struct_gep(
            struct_type,
            *struct_ptr,
            field_index as u32,
            &format!("{}_field_ptr_{}", struct_name, field_name),
        )?;

        // Store the new value into the struct field’s memory location
        self.builder
            .build_store(field_ptr, new_value)
            .map_err(|e| {
                anyhow!(
                    "Failed to store new value in struct field '{}': {}",
                    field_name,
                    e
                )
            })?;

        // Update `last_value` to the new value, useful for REPL/debugging or further codegen
        self.last_value = Some(new_value);

        // Optionally load the value back from the field pointer (e.g., for expression value)
        self.builder.build_load(
            field_type,
            field_ptr,
            &format!("{}_field_val_{}", struct_name, field_name),
        )?;

        Ok(())
    }
}
