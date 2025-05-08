use super::CodeGenerator;
use crate::ast::VectorLiteral;
use anyhow::Result;
use inkwell::{
    types::BasicType,
    values::{BasicValueEnum, PointerValue},
};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_literal(
        &mut self,
        vector_literal: &VectorLiteral,
    ) -> Result<BasicValueEnum<'ctx>> {
        let mut element_values = Vec::new();

        let element_type = self.get_llvm_type(&vector_literal.element_type)?;

        for element_expr in vector_literal.elements.iter() {
            let value = self.compile_expression(element_expr)?;
            if value.get_type() != element_type {
                return Err(anyhow::anyhow!("Vector elements must be of the same type"));
            }
            element_values.push(value);
        }

        let vector_type = element_type.array_type(element_values.len() as u32);
        let vector_ptr = self.builder.build_alloca(vector_type, "vector_literal")?;
        let vector_type = element_type.array_type(element_values.len() as u32);

        for (i, value) in element_values.iter().enumerate() {
            let zero = self.context.i32_type().const_zero();
            let index = self.context.i32_type().const_int(i as u64, false);
            let indices = [zero, index];
            let element_ptr = unsafe {
                self.builder.build_in_bounds_gep(
                    vector_type,
                    vector_ptr,
                    &indices,
                    &format!("vec_elem_ptr_{}", i),
                )
            }?;
            self.builder.build_store(element_ptr, *value)?;
        }

        // Return the pointer to the first element (T*)
        Ok(BasicValueEnum::PointerValue(vector_ptr))
    }
}
