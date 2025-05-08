use super::CodeGenerator;
use crate::ast::ArrayLiteral;
use anyhow::Result;
use inkwell::{types::BasicType, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_array_literal(
        &mut self,
        array_literal: &ArrayLiteral,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Make sure the array is not empty
        let first_element = array_literal
            .elements
            .first()
            .ok_or_else(|| anyhow::anyhow!("Cannot create an array from an empty literal"))?;

        // Compile the first element and get its type
        let first_value = self.compile_expression(first_element)?;
        let element_type = first_value.get_type();

        // Compile all element expressions and ensure they're of the same type
        let nr_elements = array_literal.elements.len() as u32;

        let array_type = element_type.array_type(nr_elements);

        let array_ptr = self.builder.build_alloca(array_type, "array_literal")?;

        // Store each value into the array
        for (i, value) in array_literal.elements.iter().enumerate() {
            let zero = self.context.i32_type().const_zero();
            let index = self.context.i32_type().const_int(i as u64, false);
            let indices = [zero, index];

            let element_ptr = unsafe {
                self.builder.build_in_bounds_gep(
                    array_type,
                    array_ptr,
                    &indices,
                    &format!("array_element_ptr_{}", i),
                )
            }?;

            let value = self.compile_expression(value)?;
            if value.get_type() != element_type {
                return Err(anyhow::anyhow!(
                    "Array elements must be of the same type"
                ));
            }
            self.builder.build_store(element_ptr, value)?;
        }

        let array_value = self.builder.build_load(
            array_type,
            array_ptr,
            "array_literal_value",
        )?;

        Ok(array_value)
    }
}
/*
                        let array_value = self.compile_expression(&decl.value)?;
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
                        let array_value = self.compile_expression(&decl.value)?;
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
*/
