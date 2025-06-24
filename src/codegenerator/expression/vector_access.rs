// Import the CodeGenerator implementation and required types
use super::CodeGenerator;
use crate::ast::VectorAccess;
use anyhow::{Result, anyhow};
use inkwell::{AddressSpace, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles access to a vector element, e.g., `v[i]`.
    /// Returns the LLVM value stored at that index in the vector.
    pub fn compile_vector_access(
        &mut self,
        vector_access: &VectorAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Extract the vector's name and compile the index expression
        let vector_name = &vector_access.vector_name;
        let index_expr = self.compile_expression(&vector_access.index)?;

        // Ensure the index is an integer value
        let index_value = match index_expr {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow!("Vector index must be an integer")),
        };

        // Look up the vector variable in the current scope (from the symbol table)
        let (vector_type, vector_var_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;

        // Load the actual vector pointer from the stack (points to heap-allocated vector)
        let vector_ptr = self
            .builder
            .build_load(
                self.context.ptr_type(AddressSpace::default()), // Treat as pointer type
                *vector_var_ptr, // Address on the stack holding the vector pointer
                &format!("{}_ptr_load", vector_name),
            )?
            .into_pointer_value();

        // Get the pointer to the actual data section of the vector (skipping header)
        let data_ptr = self.get_vector_data_ptr_from_vector(&vector_ptr)?;

        // Get the element type from the stored struct type information
        // Assumes the vector was allocated using a known struct layout
        let element_type = vector_type
            .into_struct_type()
            .get_field_type_at_index(0) // Expect the first field to represent element type
            .ok_or_else(|| anyhow!("Invalid vector struct type for '{}'", vector_name))?;

        // Compute the pointer to the element at the specified index
        let element_ptr = unsafe {
            self.builder.build_gep(
                element_type,  // The LLVM type of the element
                data_ptr,      // Start of data region (after header)
                &[index_value], // Offset/index
                &format!("{}_element_ptr", vector_name),
            )?
        };

        // Load the actual value stored at the computed element location
        let element_value = self
            .builder
            .build_load(element_type, element_ptr, "element_load")?;

        // Return the loaded value
        Ok(element_value)
    }
}
