use super::CodeGenerator;
use crate::ast::VectorLiteral;
use anyhow::{Result, anyhow};
use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_literal(
        &mut self,
        vector_literal: &VectorLiteral,
    ) -> Result<BasicValueEnum<'ctx>> {
        if vector_literal.elements.is_empty() {
            // For empty vectors, create a header with size 0
            let malloc_fn = self
                .module
                .get_function("malloc")
                .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

            // Allocate just the header (8 bytes for size)
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

            // Store size = 0 in the header
            self.builder
                .build_store(header_ptr, self.context.i64_type().const_int(0, false))?;

            return Ok(header_ptr.into());
        }

        let first_element = vector_literal.elements.first().unwrap();
        let first_value = self.compile_expression(first_element)?;
        let element_type = first_value.get_type();

        // Calculate element size in bytes
        let element_size = (match element_type {
            BasicTypeEnum::FloatType(ft) => self.get_float_bit_width(ft)?,
            BasicTypeEnum::IntType(it) => it.get_bit_width(),
            _ => 64,
        } + 7)
            / 8;

        let nr_elements = vector_literal.elements.len() as u32;
        let data_size = element_size * nr_elements;

        // Total size = 8 bytes for size header + data size
        let total_size = 8 + data_size;

        let malloc_fn = self
            .module
            .get_function("malloc")
            .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

        // Allocate memory for header + data
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

        // Store the size in the header (first 8 bytes)
        self.builder.build_store(
            vector_ptr,
            self.context.i64_type().const_int(nr_elements as u64, false),
        )?;

        // Get pointer to the data section (after the 8-byte header)
        let data_ptr = unsafe {
            self.builder.build_gep(
                self.context.i8_type(),
                vector_ptr,
                &[self.context.i64_type().const_int(8, false)],
                "data_ptr",
            )?
        };

        // Store all elements in the data section
        for (i, element_expr) in vector_literal.elements.iter().enumerate() {
            let value = self.compile_expression(element_expr)?;
            let idx = self.context.i64_type().const_int(i as u64, false);

            let element_ptr = unsafe {
                self.builder
                    .build_gep(element_type, data_ptr, &[idx], "element_ptr")?
            };

            self.builder.build_store(element_ptr, value)?;
        }

        println!(
            "Vector literal pointer: {:?}, elements: {}, total size: {}",
            vector_ptr, nr_elements, total_size
        );

        // Return the pointer to the entire structure (header + data)
        Ok(vector_ptr.into())
    }
}
