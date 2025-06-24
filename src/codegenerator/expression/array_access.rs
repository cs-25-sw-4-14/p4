use super::CodeGenerator;
use crate::ast::ArrayAccess;
use anyhow::Result;
use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles an array element access expression, e.g. `arr[index]`.
    ///
    /// Returns the loaded value at the specified index.
    pub fn compile_array_access(
        &mut self,
        array_access: &ArrayAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Get the variable name of the array
        let array_name = &array_access.array_name;

        // Compile the index expression, must evaluate to an integer
        let index_value = self.compile_expression(&array_access.index)?;

        // Lookup the array variable's type and pointer from the symbol table
        let (array_type, array_ptr) = self
            .variables
            .get(array_name)
            .ok_or_else(|| anyhow::anyhow!("Array variable '{}' not found", array_name))?;

        // Ensure the index is an integer value
        let index_value = match index_value {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow::anyhow!("Array index must be an integer")),
        };

        // Constant zero index for the first dimension (arrays are pointer to element)
        let zero = self.context.i32_type().const_zero();

        // Indices used for GEP (GetElementPtr):
        // The first index zero dereferences the pointer,
        // the second is the index into the array.
        let indices = [zero, index_value];

        // Use unsafe build_gep to create a pointer to the desired array element.
        // Parameters:
        // - array_type: the type of the array (e.g. [10 x i32])
        // - array_ptr: pointer to the array
        // - indices: [0, index] to access the element at position 'index'
        // - name: debug/name hint for the instruction
        let element_ptr = unsafe {
            self.builder.build_gep(
                *array_type,
                *array_ptr,
                &indices,
                &format!("array_index_{}", array_name),
            )
        }?;

        // Extract the element type from the array type for loading
        let element_type = match array_type {
            BasicTypeEnum::ArrayType(array_ty) => array_ty.get_element_type(),
            _ => {
                return Err(anyhow::anyhow!(
                    "Expected an array type for variable '{}'",
                    array_name
                ));
            }
        };

        // Load the value stored at the computed element pointer
        let element_value = self.builder.build_load(
            element_type,
            element_ptr,
            &format!("array_element_{}", array_name),
        )?;

        // Return the loaded element value
        Ok(element_value)
    }
}
