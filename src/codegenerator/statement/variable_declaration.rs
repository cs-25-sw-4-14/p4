use super::CodeGenerator;
use crate::ast::{Expression, Type, VariableDeclaration};
use anyhow::{Result, anyhow};
use inkwell::types::{BasicTypeEnum, VectorType};

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
            Type::Vector(elem_type, _) => {
                let vector_literal = match &decl.value {
                    Expression::VectorLiteral(lit) => lit,
                    _ => return Err(anyhow!("Expected vector literal for vector variable")),
                };

                let llvm_elem_type = self.get_llvm_type(elem_type)?;

                let size = vector_literal.elements.len();

                let vector_type: VectorType<'_> = match llvm_elem_type {
                    BasicTypeEnum::IntType(t) => t.vec_type(size as u32),
                    _ => return Err(anyhow!("Unsupported element type")),
                };

                let vector_ptr = self.builder.build_alloca(vector_type, "vector")?;

                for (i, elem_expr) in vector_literal.elements.iter().enumerate().skip(1) {
                    let elem_value = self.compile_expression(elem_expr)?;
                    let elem_ptr = unsafe {
                        self.builder.build_in_bounds_gep(
                            vector_type,
                            vector_ptr,
                            &[
                                self.context.i32_type().const_zero(),
                                self.context.i32_type().const_int(i as u64, false),
                            ],
                            &format!("vector_elem_{}", i),
                        )
                    }?;
                    self.builder.build_store(elem_ptr, elem_value)?;
                }
            }
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
