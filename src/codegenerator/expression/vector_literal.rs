use super::CodeGenerator;
use crate::ast::VectorLiteral;
use anyhow::{Result, anyhow};
use inkwell::{AddressSpace, types::BasicTypeEnum, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_literal(
        &mut self,
        vector_literal: &VectorLiteral,
    ) -> Result<BasicValueEnum<'ctx>> {
        let first_element = vector_literal
            .elements
            .first()
            .ok_or_else(|| anyhow!("Vector literal must have at least one element"))?;

        let first_value = self.compile_expression(first_element)?;
        let element_type = first_value.get_type();
        let nr_elements = vector_literal.elements.len() as u32;

        let vector_type = match element_type {
            BasicTypeEnum::FloatType(ft) => Ok(ft.vec_type(nr_elements)),
            BasicTypeEnum::IntType(it) => Ok(it.vec_type(nr_elements)),
            BasicTypeEnum::PointerType(pt) => Ok(pt.vec_type(nr_elements)),
            BasicTypeEnum::StructType(_) => Ok(self
                .context
                .ptr_type(AddressSpace::default())
                .vec_type(nr_elements)),
            _ => Err(anyhow!("Unsupported vector element type")),
        }?;

        let mut vector_value = vector_type.get_undef();

        for (i, element_expr) in vector_literal.elements.iter().enumerate() {
            let value = self.compile_expression(element_expr)?;

            if value.get_type() != element_type {
                return Err(anyhow!("Vector elements must be of the same type"));
            }

            let idx = self.context.i32_type().const_int(i as u64, false);

            vector_value = self.builder.build_insert_element(
                vector_value,
                value,
                idx,
                &format!("vector_insert_{}", i),
            )?;
        }

        Ok(vector_value.into())
    }
}
