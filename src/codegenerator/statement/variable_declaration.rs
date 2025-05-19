use super::CodeGenerator;
use crate::ast::{Type, VariableDeclaration};
use anyhow::{Result, anyhow};
use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};

impl CodeGenerator<'_> {
    pub fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        let value = self.compile_expression(&decl.value)?;
        let var_name = &decl.name;

        match &decl.var_type {
            Type::String => {
                let string_ptr = value.into_pointer_value();

                let str_len: BasicValueEnum = self
                    .heap_sizes
                    .get(&value.into_pointer_value())
                    .ok_or_else(|| anyhow::anyhow!("String length not found in heap_sizes"))?
                    .clone()
                    .into();
                // Call malloc with size = length + 1 (for null terminator)
                let i8_ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
                let malloc_fn = self
                    .module
                    .get_function("malloc")
                    .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

                let raw_ptr = self
                    .builder
                    .build_call(malloc_fn, &[str_len.into()], "malloc_call")?
                    .try_as_basic_value()
                    .left()
                    .unwrap()
                    .into_pointer_value();

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
                        self.builder
                            .build_pointer_cast(string_ptr, i8_ptr_type, "cast")?
                            .into(),
                        str_len.into(),
                        self.context.bool_type().const_zero().into(), // not volatile
                    ],
                    "memcpy_call",
                )?;

                let llvm_var_type = i8_ptr_type;
                let var_ptr = self.builder.build_alloca(llvm_var_type, var_name)?;
                self.builder.build_store(var_ptr, raw_ptr)?;

                self.variables
                    .insert(var_name.clone(), (llvm_var_type.into(), var_ptr));
                return Ok(());
            }
            Type::Vector(_, _) => {
                let nr_elements = self
                    .heap_sizes
                    .get(&value.into_pointer_value())
                    .ok_or_else(|| anyhow::anyhow!("Vector length not found in heap_sizes"))?
                    .get_zero_extended_constant()
                    .ok_or_else(|| anyhow::anyhow!("Failed to get zero extended constant"))?;

                let ptr_type = self.context.ptr_type(inkwell::AddressSpace::default());
                let size_type = self.context.i64_type();
                let vector_struct_type = self
                    .context
                    .struct_type(&[ptr_type.into(), size_type.into()], false);

                let var_ptr = self.builder.build_alloca(vector_struct_type, var_name)?;

                let data_ptr_ptr = self.builder.build_struct_gep(
                    vector_struct_type,
                    var_ptr,
                    0,
                    &format!("{}_data_ptr", var_name),
                )?;
                self.builder.build_store(data_ptr_ptr, value)?;

                let size_ptr = self.builder.build_struct_gep(
                    vector_struct_type,
                    var_ptr,
                    1,
                    &format!("{}_size", var_name),
                )?;
                let length = self.context.i64_type().const_int(nr_elements, false);
                self.builder.build_store(size_ptr, length)?;

                self.heap_sizes.insert(var_ptr, length);

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
