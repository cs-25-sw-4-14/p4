use super::CodeGenerator;
use crate::ast::{Type, VariableDeclaration};
use anyhow::Result;
use inkwell::{AddressSpace, types::BasicTypeEnum};

impl CodeGenerator<'_> {
    /// Compiles a variable declaration.
    ///
    /// Handles normal variables and vectors differently.
    /// Example:
    ///   let x: i32 = 5;
    ///   let v: Vector<i32> = ...;
    pub fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        // Compile the initializer expression to get the initial value
        let value = self.compile_expression(&decl.value)?;
        let var_name = &decl.name;

        match &decl.var_type {
            // Special handling for vector types (e.g., Vector<T>)
            Type::Vector(element_type, _) => {
                // The value should be a pointer to the vector data structure
                let vector_ptr = value.into_pointer_value();

                // Allocate stack memory to store the vector pointer (a pointer to the vector struct)
                let var_ptr = self
                    .builder
                    .build_alloca(self.context.ptr_type(AddressSpace::default()), var_name)?;

                // Store the vector pointer in the allocated space
                self.builder.build_store(var_ptr, vector_ptr)?;

                // Prepare types for the vector struct: element type and size type (i64)
                let size_type = self.context.i64_type();
                let llvm_element_type = self.get_llvm_type(element_type)?;

                // Define the LLVM struct type for the vector:
                // struct { element_type[], i64 size }
                let vector_struct_type = self
                    .context
                    .struct_type(&[llvm_element_type.into(), size_type.into()], false);

                let llvm_struct_type = BasicTypeEnum::StructType(vector_struct_type);

                // Insert the vector variable into the symbol table:
                // maps variable name to (type, pointer)
                self.variables
                    .insert(var_name.clone(), (llvm_struct_type, var_ptr));

                return Ok(()); // Early return since vector handled separately
            }

            // General case for other variable types
            _ => {
                // Get the LLVM type for the variable type
                let llvm_var_type = self.get_llvm_type(&decl.var_type)?;

                // Allocate stack memory for the variable
                let var_ptr = self.builder.build_alloca(llvm_var_type, var_name)?;

                // Store the initial value in the allocated space
                self.builder.build_store(var_ptr, value)?;

                // Insert into symbol table for later use
                self.variables
                    .insert(var_name.clone(), (llvm_var_type, var_ptr));

                Ok(())
            }
        }
    }
}
