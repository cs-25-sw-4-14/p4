use super::CodeGenerator;
use crate::ast::VectorAccess;
use anyhow::{Result, anyhow};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_access(
        &mut self,
        vector_access: &VectorAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        let vector_name = &vector_access.vector_name;
        
        let index_expr = self.compile_expression(&vector_access.index)?;
        let index_value = match index_expr {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow!("Vector index must be an integer")),
        };
        
        let (vector_type, vector_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;
        
        let vector_value = self.builder.build_load(*vector_type, *vector_ptr, vector_name)?;
        
        let vector_value = match vector_value {
            BasicValueEnum::VectorValue(val) => val,
            _ => return Err(anyhow!("Variable '{}' is not a vector", vector_name)),
        };
        
        let element_value = self.builder.build_extract_element(
            vector_value,
            index_value,
            &format!("{}_element", vector_name)
        )?;
        
        Ok(element_value)
    }
}
