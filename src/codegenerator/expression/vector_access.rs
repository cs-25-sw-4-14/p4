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

        let (vector_type, vector_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;

        let element_type = vector_type
            .into_struct_type()
            .get_field_type_at_index(0)
            .unwrap();

        println!("Element type: {:?}", element_type);

        let loaded_vector_ptr = self
            .builder
            .build_load(
                self.context.ptr_type(AddressSpace::default()),
                *vector_ptr,
                &format!("{}_loaded_ptr", vector_name),
            )?
            .into_pointer_value();

        let element_ptr = unsafe {
            self.builder.build_gep(
                element_type, // The pointee type
                loaded_vector_ptr,
                &[index_value],
                &format!("{}_element_ptr", vector_name),
            )?
        };

        let element_value = self
            .builder
            .build_load(element_type, element_ptr, "element_load")?;

        Ok(element_value)
    }
}
