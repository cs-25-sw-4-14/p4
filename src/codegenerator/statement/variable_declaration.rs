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
                let string_ptr = value.into_pointer_value();

                // Get the length from the heap-allocated string
                let str_len = self.get_string_length(string_ptr)?;

                // Calculate total size needed (8 bytes for length + string data + null terminator)
                let total_size = self.builder.build_int_add(
                    str_len,
                    self.context.i64_type().const_int(8 + 1, false), // header + null terminator
                    "total_size",
                )?;

                // Allocate new memory for the copy
                let malloc_fn = self
                    .module
                    .get_function("malloc")
                    .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

                let new_ptr = self
                    .builder
                    .build_call(malloc_fn, &[total_size.into()], "malloc_call")?
                    .try_as_basic_value()
                    .left()
                    .unwrap()
                    .into_pointer_value();

                // Copy the entire string struct (header + data)
                let memcpy_fn = self.module.get_function("llvm.memcpy.p0.p0.i64").unwrap();

                self.builder.build_call(
                    memcpy_fn,
                    &[
                        new_ptr.into(),                               // dest
                        string_ptr.into(),                            // src
                        total_size.into(),                            // size
                        self.context.bool_type().const_zero().into(), // volatile = false
                    ],
                    "memcpy_string_var",
                )?;

                // Store the pointer in a variable
                let var_ptr = self
                    .builder
                    .build_alloca(self.context.ptr_type(AddressSpace::default()), var_name)?;

                self.builder.build_store(var_ptr, new_ptr)?;

                self.variables.insert(
                    var_name.clone(),
                    (
                        self.context.ptr_type(AddressSpace::default()).into(),
                        var_ptr,
                    ),
                );

                return Ok(());
            }
            Type::Vector(element_type, _) => {
                let vector_ptr = value.into_pointer_value();

                let var_ptr = self
                    .builder
                    .build_alloca(self.context.ptr_type(AddressSpace::default()), var_name)?;

                self.builder.build_store(var_ptr, vector_ptr)?;

                let size_type = self.context.i64_type();
                let llvm_element_type = self.get_llvm_type(element_type)?;
                let vector_struct_type = self
                    .context
                    .struct_type(&[llvm_element_type.into(), size_type.into()], false);

                let llvm_struct_type = BasicTypeEnum::StructType(vector_struct_type);

                self.variables
                    .insert(var_name.clone(), (llvm_struct_type, var_ptr));

                return Ok(());
            }
            _ => {
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
