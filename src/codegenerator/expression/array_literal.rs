use super::CodeGenerator;
use crate::ast::ArrayLiteral;
use anyhow::Result;
use inkwell::{types::BasicType, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles an array literal expression, e.g. `[a, b, c]`.
    ///
    /// Allocates stack memory for the array and stores each element.
    /// Returns the loaded array value.
    pub fn compile_array_literal(
        &mut self,
        array_literal: &ArrayLiteral,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Ensure the array literal is not empty (empty arrays are not supported here)
        let first_element = array_literal
            .elements
            .first()
            .ok_or_else(|| anyhow::anyhow!("Cannot create an array from an empty literal"))?;

        // Compile the first element to get its LLVM type for the array element type
        let first_value = self.compile_expression(first_element)?;
        let element_type = first_value.get_type();

        // Number of elements in the array literal
        let nr_elements = array_literal.elements.len() as u32;

        // Create LLVM array type of element_type with nr_elements size
        let array_type = element_type.array_type(nr_elements);

        // Allocate stack memory for the entire array (like `alloca [N x T]`)
        let array_ptr = self.builder.build_alloca(array_type, "array_literal")?;

        // Loop over each element to compile and store it in the allocated array
        for (i, value) in array_literal.elements.iter().enumerate() {
            // Prepare GEP indices to get pointer to the i-th element:
            // First zero to dereference the pointer to the array,
            // second is the element index.
            let zero = self.context.i32_type().const_zero();
            let index = self.context.i32_type().const_int(i as u64, false);
            let indices = [zero, index];

            // Compute pointer to the i-th element of the array
            let element_ptr = unsafe {
                self.builder.build_in_bounds_gep(
                    array_type,
                    array_ptr,
                    &indices,
                    &format!("array_element_ptr_{}", i),
                )
            }?;

            // Compile the current element expression
            let value = self.compile_expression(value)?;

            // Check that all elements have the same type as the first element
            if value.get_type() != element_type {
                return Err(anyhow::anyhow!("Array elements must be of the same type"));
            }

            // Store the compiled element into the array at the computed pointer
            self.builder.build_store(element_ptr, value)?;
        }

        // Load the whole array value from the allocated array pointer
        let array_value = self
            .builder
            .build_load(array_type, array_ptr, "array_literal_value")?;

        // Return the loaded array value
        Ok(array_value)
    }
}
