use super::CodeGenerator;
use crate::ast::ArrayLiteral;
use anyhow::Result;
use inkwell::{types::BasicType, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_array_literal(
        &mut self,
        array_literal: &Box<ArrayLiteral>,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Make sure the array is not empty
        let first_element = array_literal
            .elements
            .first()
            .ok_or_else(|| anyhow::anyhow!("Cannot create an array from an empty literal"))?;

        // Compile the first element and get its type
        let first_value = self.compile_expression(first_element)?;
        let element_type = first_value.get_type();

        // Compile all element expressions and ensure they're of the same type
        let mut element_values = Vec::new();
        element_values.push(first_value);

        for element_expr in array_literal.elements.iter().skip(1) {
            let value = self.compile_expression(element_expr)?;
            // Optionally check type equality here
            if value.get_type() != element_type {
                return Err(anyhow::anyhow!("Array elements must be of the same type"));
            }
            element_values.push(value);
        }

        // Create LLVM array type and allocate it
        let array_type = element_type.array_type(element_values.len() as u32);
        let array_ptr = self.builder.build_alloca(array_type, "array_literal")?;

        // Store each value into the array
        for (i, value) in element_values.iter().enumerate() {
            let zero = self.context.i32_type().const_zero();
            let index = self.context.i32_type().const_int(i as u64, false);
            let indices = [zero, index];

            let element_ptr = unsafe {
                self.builder.build_in_bounds_gep(
                    array_type,
                    array_ptr,
                    &indices,
                    &format!("array_element_ptr_{}", i),
                )
            }?;

            self.builder.build_store(element_ptr, *value)?;
        }

        Ok(BasicValueEnum::PointerValue(array_ptr))
    }
}
