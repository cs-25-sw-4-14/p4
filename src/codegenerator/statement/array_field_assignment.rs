use crate::{ast::ArrayFieldAssignment, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};

impl CodeGenerator<'_> {
    /// Compiles an assignment to a specific index of an array.
    ///
    /// Example: `array[2] = 10;`
    pub fn compile_array_field_assignment(&mut self, assign: &ArrayFieldAssignment) -> Result<()> {
        // Compile the value to assign into LLVM IR (e.g., 10 in array[2] = 10)
        let new_value = self.compile_expression(&assign.value)?;

        // Get the name of the target array (e.g., "array")
        let array_name = &assign.target.clone();

        // Compile the index expression (e.g., 2 in array[2] = 10)
        let index_value = self.compile_expression(&assign.field_index)?;

        // Look up the array variable in the symbol table (variable map)
        // Should return a tuple: (array_type, pointer_to_array)
        let (array_type, array_ptr) = self
            .variables
            .get(array_name)
            .ok_or_else(|| anyhow!("Array variable '{}' not found", array_name))?;

        // Ensure the index is an integer value, as required by LLVM for indexing
        let index_value = match index_value {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow!("Array index must be an integer")),
        };

        // The first index (0) is for the base pointer of the array
        let zero = self.context.i32_type().const_zero();

        // The second index is the actual dynamic index (e.g., 2 in array[2])
        let indices = [zero, index_value];

        // Compute a pointer to the specific element in the array using GEP (GetElementPtr)
        let element_ptr = unsafe {
            self.builder.build_gep(
                *array_type,                 // The type of the array
                *array_ptr,                  // The pointer to the array
                &indices,                    // The GEP indices (base + offset)
                &format!("array_index_{}", array_name), // Name of this temporary
            )
        }?;

        // Extract the element type from the array type for later use
        let element_type = match array_type {
            BasicTypeEnum::ArrayType(array_ty) => array_ty.get_element_type(),
            _ => {
                return Err(anyhow!(
                    "Expected an array type for variable '{}'",
                    array_name
                ));
            }
        };

        // Store the new value into the computed element pointer
        self.builder
            .build_store(element_ptr, new_value)
            .map_err(|e| anyhow!("Failed to store new value in array '{}': {}", array_name, e))?;

        // Track the last computed value (useful for REPL-style eval or debugging)
        self.last_value = Some(new_value);

        // Optionally load the value back (not strictly needed unless value is used afterward)
        self.builder.build_load(
            element_type,                 // Type of element to load
            element_ptr,                  // Pointer to the element
            &format!("array_element_{}", array_name), // Name for IR
        )?;

        Ok(())
    }
}
