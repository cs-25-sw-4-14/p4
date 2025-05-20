use super::CodeGenerator;
use crate::ast::{Type, VariableDeclaration};
use anyhow::{Result, anyhow};
use inkwell::{AddressSpace, types::BasicTypeEnum};

impl CodeGenerator<'_> {
    pub fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        let value = self.compile_expression(&decl.value)?;
        let var_name = &decl.name;

        match &decl.var_type {
            Type::String => {
                // String handling - allocate a simple pointer
                let string_ptr = value.into_pointer_value();
                
                // 1. Get the length of the string dynamically
                let str_len = self
                    .builder
                    .build_call(
                        self.module.get_function("strlen").unwrap(),
                        &[string_ptr.into()],
                        "strlen_call",
                    )?
                    .try_as_basic_value()
                    .left()
                    .unwrap()
                    .into_int_value();

                // 2. Call malloc with size = length (we'll copy the content)
                let i8_ptr_type = self.context.ptr_type(AddressSpace::default());
                let malloc_fn = self
                    .module
                    .get_function("malloc")
                    .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

                let malloc_size = str_len; // No need to add 1 if original already has null terminator
                
                let raw_ptr = self
                    .builder
                    .build_call(malloc_fn, &[malloc_size.into()], "malloc_call")?
                    .try_as_basic_value()
                    .left()
                    .unwrap()
                    .into_pointer_value();

                // 3. Copy the string content
                let memcpy_fn = self
                    .module
                    .get_function("llvm.memcpy.p0.p0.i64")
                    .unwrap_or_else(|| {
                        self.module.add_function(
                            "llvm.memcpy.p0.p0.i64",
                            self.context.void_type().fn_type(
                                &[
                                    i8_ptr_type.into(),              // dest
                                    i8_ptr_type.into(),              // src
                                    self.context.i64_type().into(),  // len
                                    self.context.bool_type().into(), // isVolatile
                                ],
                                false,
                            ),
                            None,
                        )
                    });

                self.builder.build_call(
                    memcpy_fn,
                    &[
                        raw_ptr.into(),
                        string_ptr.into(),  // Already a pointer, no need to cast
                        str_len.into(),
                        self.context.bool_type().const_zero().into(), // not volatile
                    ],
                    "memcpy_call",
                )?;

                // 4. Store the string pointer on the stack
                let var_ptr = self.builder.build_alloca(i8_ptr_type, var_name)?;
                self.builder.build_store(var_ptr, raw_ptr)?;

                // 5. Store the variable metadata in the variables map
                self.variables
                    .insert(var_name.clone(), (i8_ptr_type.into(), var_ptr));
                
                return Ok(());
            }
            Type::Vector(v_type, _) => {
                // For vectors, don't allocate new space - the expression should already 
                // have created the vector struct on the heap
                let ptr_type = self.context.ptr_type(AddressSpace::default());
                let len_type = self.context.i64_type();

                // Define vector struct type with data pointer and length
                let vector_struct_type = self
                    .context
                    .struct_type(&[ptr_type.into(), len_type.into()], false);

                // Create a stack allocation to store the vector struct pointer
                let llvm_struct_type = BasicTypeEnum::StructType(vector_struct_type);
                
                // value should already be a pointer to our struct from compile_vector_literal
                let struct_ptr = value.into_pointer_value();
                
                // Create a stack allocation for this pointer
                let var_alloca = self.builder.build_alloca(llvm_struct_type, var_name)?;
                
                // Store the struct pointer in our stack variable
                self.builder.build_store(var_alloca, struct_ptr)?;
                
                // Register the variable
                self.variables
                    .insert(var_name.clone(), (llvm_struct_type, var_alloca));

                return Ok(());
            }
            _ => {
                // Handle other types normally
                let llvm_var_type = self.get_llvm_type(&decl.var_type)?;
                let var_ptr = self.builder.build_alloca(llvm_var_type, var_name)?;
                self.builder.build_store(var_ptr, value)?;
                self.variables
                    .insert(var_name.clone(), (llvm_var_type, var_ptr));

                Ok(())
            }
        }
    }
}
