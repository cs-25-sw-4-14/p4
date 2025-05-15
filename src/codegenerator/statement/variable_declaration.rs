use super::CodeGenerator;
use crate::ast::{Type, VariableDeclaration};
use anyhow::{Result, anyhow};
use inkwell::types::BasicTypeEnum;

impl CodeGenerator<'_> {
    pub fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        let value = self.compile_expression(&decl.value)?;

        let var_type = &decl.var_type;
        let var_name = &decl.name;

        let llvm_var_type = self.get_llvm_type(var_type)?;

        let array_ptr = self
            .builder
            .build_alloca(llvm_var_type, var_name)
            .map_err(|e| {
                anyhow!(
                    "Failed to allocate memory for variable '{}': {}",
                    var_name,
                    e
                )
            })?;

        match var_type {
            Type::Array(elem_type, size) => {
                let array_literal_ptr = self.compile_expression(&decl.value)?.into_pointer_value();
                let llvm_elem_type = self.get_llvm_type(elem_type)?;
                let array_llvm_type = llvm_var_type.into_array_type();

                for i in 0..*size {
                    let index = self.context.i32_type().const_int(i, false);
                    let literal_elem_ptr = unsafe {
                        self.builder.build_in_bounds_gep(
                            array_llvm_type,
                            array_literal_ptr,
                            &[self.context.i32_type().const_zero(), index],
                            &format!("{}_literal_elem_{}", var_name, i),
                        )
                    }?;
                    let literal_elem_value = self.builder.build_load(
                        llvm_elem_type,
                        literal_elem_ptr,
                        &format!("{}_literal_val_{}", var_name, i),
                    )?;

                    let var_elem_ptr = unsafe {
                        self.builder.build_in_bounds_gep(
                            array_llvm_type,
                            array_ptr,
                            &[self.context.i32_type().const_zero(), index],
                            &format!("{}_elem_{}", var_name, i),
                        )
                    }?;
                    self.builder.build_store(var_elem_ptr, literal_elem_value)?;
                }
            }
            Type::Vector(elem_type, _size) => {
                let vec_val = self.compile_expression(&decl.value)?.into_vector_value();
                let new_size = vec_val.get_type().get_size();

                let llvm_elem_type = self.get_llvm_type(elem_type)?;

                match llvm_elem_type {
                    BasicTypeEnum::IntType(int_type) => {
                        let vector_llvm_type = int_type.vec_type(new_size);

                        let vec_ptr = self.builder.build_alloca(vector_llvm_type, var_name)?;
                        self.builder.build_store(vec_ptr, vec_val)?;

                        self.variables
                            .insert(var_name.to_string(), (vector_llvm_type.into(), vec_ptr));
                    }
                    BasicTypeEnum::FloatType(float_type) => {
                        let vector_llvm_type = float_type.vec_type(new_size);

                        let vec_ptr = self.builder.build_alloca(vector_llvm_type, var_name)?;

                        self.builder.build_store(vec_ptr, vec_val)?;

                        self.variables
                            .insert(var_name.to_string(), (vector_llvm_type.into(), vec_ptr));
                    }
                    _ => return Err(anyhow!("Unsupported vector type")),
                }
            }
            Type::Custom(_) => {
                let struct_value = self.compile_expression(&decl.value)?;
                self.builder.build_store(array_ptr, struct_value)?;
            }
            _ => {
                self.builder.build_store(array_ptr, value)?;
            }
        }

        self.variables
            .insert(var_name.clone(), (llvm_var_type, array_ptr));

        Ok(())
    }
}
