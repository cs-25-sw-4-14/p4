use super::CodeGenerator;
use crate::ast::VectorAccess;
use anyhow::{Result, anyhow};
use inkwell::{AddressSpace, values::BasicValueEnum};

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

        let (vector_type, vector_var_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;

        // Load the actual vector pointer from the variable
        let vector_ptr = self
            .builder
            .build_load(
                self.context.ptr_type(AddressSpace::default()),
                *vector_var_ptr,
                &format!("{}_ptr_load", vector_name),
            )?
            .into_pointer_value();

        let data_ptr = self.get_vector_data_ptr_from_vector(&vector_ptr)?;

        // Extract element type from the stored struct type information
        let element_type = vector_type
            .into_struct_type()
            .get_field_type_at_index(0)
            .ok_or_else(|| anyhow!("Invalid vector struct type for '{}'", vector_name))?;

        
        // Get the specific element
        let element_ptr = unsafe {
            self.builder.build_gep(
                element_type,
                data_ptr,
                &[index_value],
                &format!("{}_element_ptr", vector_name),
            )?
        };

        // Load and return the element value
        let element_value = self
            .builder
            .build_load(element_type, element_ptr, "element_load")?;

        Ok(element_value)
    }
}
