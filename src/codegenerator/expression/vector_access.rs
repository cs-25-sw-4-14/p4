use super::CodeGenerator;
use crate::ast::VectorAccess;
use anyhow::{Result, anyhow};
use inkwell::{AddressSpace, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_access(
        &mut self,
        vector_access: &VectorAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        let index_expr = self.compile_expression(&vector_access.index)?;
        let index_value = match index_expr {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow!("Vector index must be an integer")),
        };
        
        // Get vector variable from scope
        let vector_name = &vector_access.vector_name;
        let (vector_type, vector_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;

        // Compile the index expression

        // Get the struct type
        let struct_type = vector_type.into_struct_type();

        // First, extract the data pointer (field 0 of the struct)
        let data_ptr_ptr = self.builder.build_struct_gep(
            struct_type,
            *vector_ptr,
            0,
            &format!("{}_data_ptr_ptr", vector_name),
        )?;

        // Load the data pointer
        let data_ptr = self.builder.build_load(
            self.context.ptr_type(AddressSpace::default()),
            data_ptr_ptr,
            &format!("{}_data_ptr", vector_name),
        )?;

        // Determine the element type - this depends on your language's type system
        // For now, we'll assume it's a pointer type
        let element_type = self.context.ptr_type(AddressSpace::default());

        // Access the element at the specified index
        let element_ptr = unsafe {
            self.builder.build_gep(
                element_type,
                data_ptr.into_pointer_value(),
                &[index_value],
                &format!("{}_element_ptr", vector_name),
            )?
        };

        // Load the element value
        let element_value = self
            .builder
            .build_load(element_type, element_ptr, "element_load")?;

        Ok(element_value)
    }
}
