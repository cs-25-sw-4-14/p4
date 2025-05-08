use super::CodeGenerator;
use crate::ast::VectorAccess;
use anyhow::{Result, anyhow};
use inkwell::{
    types::BasicTypeEnum,
    values::{BasicValueEnum, VectorValue},
};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_access(
        &mut self,
        vector_access: &VectorAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        let vector_name = &vector_access.vector_name;
        let index_value = self.compile_expression(&vector_access.index)?;

        let (vector_type, vector_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow::anyhow!("Vector variable '{}' not found", vector_name))?;

        let vector_value = self.builder.build_load(*vector_type, *vector_ptr, vector_name)?;

        let element_value = vector_value.const_extract_element(index_value);

        let index_value = match index_value {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow::anyhow!("Vector index must be an integer")),
        };

        println!(
            "Vector access: {}[{}] = {:?}",
            vector_name, index_value, element_value
        );
        Ok(element_value)
    }
}
