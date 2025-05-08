use super::CodeGenerator;
use crate::ast::ArrayAccess;
use anyhow::Result;
use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_array_access(
        &mut self,
        array_access: &ArrayAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        let array_name = &array_access.array_name;
        let index_value = self.compile_expression(&array_access.index)?;
        println!("Array access: {}[{}]", array_name, index_value);

        let (array_type, array_ptr) = self
            .variables
            .get(array_name)
            .ok_or_else(|| anyhow::anyhow!("Array variable '{}' not found", array_name))?;

        println!("Array type: {:?}", array_type);

        let index_value = match index_value {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow::anyhow!("Array index must be an integer")),
        };

        let zero = self.context.i32_type().const_zero();

        // Create GEP (GetElementPointer) instruction to access the array element
        let indices = [zero, index_value];

        // Using the correct pointer directly array_ptr is already a PointerValue
        let element_ptr = unsafe {
            self.builder.build_gep(
                *array_type,                            // First parameter is the basic type
                *array_ptr,                             // Second parameter is the pointer value
                &indices,                               // Third parameter is the indices
                &format!("array_index_{}", array_name), // Fourth parameter is the name
            )
        }?;

        let element_type = match array_type {
            BasicTypeEnum::ArrayType(array_ty) => array_ty.get_element_type(),
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected an array type for variable '{}'",
                    array_name
                ));
            }
        };

        // Load the value from the calculated address
        let element_value = self.builder.build_load(
            element_type,                             // First parameter is the type
            element_ptr,                              // Second parameter is the pointer
            &format!("array_element_{}", array_name), // Third parameter is the name
        )?;

        Ok(element_value)
    }
}
