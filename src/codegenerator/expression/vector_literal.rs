// Import the necessary module and types
use super::CodeGenerator;
use crate::ast::VectorLiteral;
use anyhow::{Result, anyhow};
use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a vector literal into LLVM IR.
    /// Returns a pointer to a memory block that includes:
    /// - A header containing the number of elements (8 bytes)
    /// - Followed by the serialized element values.
    pub fn compile_vector_literal(
        &mut self,
        vector_literal: &VectorLiteral,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Case: Empty vector
        if vector_literal.elements.is_empty() {
            // Lookup the 'malloc' function to allocate memory
            let malloc_fn = self
                .module
                .get_function("malloc")
                .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

            // Allocate 8 bytes for the header (to store size = 0)
            let header_ptr = self
                .builder
                .build_call(
                    malloc_fn,
                    &[self.context.i64_type().const_int(8, false).into()],
                    "malloc_empty_vector",
                )?
                .try_as_basic_value()
                .left()
                .unwrap()
                .into_pointer_value();

            // Store 0 as the size of the vector in the first 8 bytes
            self.builder
                .build_store(header_ptr, self.context.i64_type().const_int(0, false))?;

            // Return the pointer to the allocated memory (only header)
            return Ok(header_ptr.into());
        }

        // Get the first element to determine the type of elements in the vector
        let first_element = vector_literal.elements.first().unwrap();
        let first_value = self.compile_expression(first_element)?;
        let element_type = first_value.get_type();

        // Compute the byte size of a single element depending on its type
        let element_size = (match element_type {
            BasicTypeEnum::FloatType(ft) => self.get_float_bit_width(ft)?, // e.g. 32 or 64 bits
            BasicTypeEnum::IntType(it) => it.get_bit_width(),              // e.g. 32 or 64 bits
            _ => 64, // Default to 64 bits (8 bytes) for unsupported types
        } + 7) / 8; // Round up to nearest byte

        let nr_elements = vector_literal.elements.len() as u32;
        let data_size = element_size * nr_elements;

        // Total size = 8-byte header + data bytes for all elements
        let total_size = 8 + data_size;

        // Lookup 'malloc' again to allocate memory
        let malloc_fn = self
            .module
            .get_function("malloc")
            .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

        // Allocate memory for header + element data
        let vector_ptr = self
            .builder
            .build_call(
                malloc_fn,
                &[self
                    .context
                    .i64_type()
                    .const_int(total_size as u64, false)
                    .into()],
                "malloc_vector",
            )?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // Store the number of elements in the first 8 bytes (header)
        self.builder.build_store(
            vector_ptr,
            self.context.i64_type().const_int(nr_elements as u64, false),
        )?;

        // Compute pointer to start of the data section (after 8-byte header)
        let data_ptr = unsafe {
            self.builder.build_gep(
                self.context.i8_type(), // Operate at byte level
                vector_ptr,
                &[self.context.i64_type().const_int(8, false)],
                "data_ptr",
            )?
        };

        // Store each element in its appropriate position in memory
        for (i, element_expr) in vector_literal.elements.iter().enumerate() {
            let value = self.compile_expression(element_expr)?;
            let idx = self.context.i64_type().const_int(i as u64, false);

            // Calculate element pointer based on element type and index
            let element_ptr = unsafe {
                self.builder
                    .build_gep(element_type, data_ptr, &[idx], "element_ptr")?
            };

            // Store the compiled value at the computed memory location
            self.builder.build_store(element_ptr, value)?;
        }

        // Return a pointer to the allocated vector (including header)
        Ok(vector_ptr.into())
    }
}
