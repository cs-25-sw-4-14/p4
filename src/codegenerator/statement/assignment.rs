use crate::{ast::Assignment, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};

impl CodeGenerator<'_> {
    pub fn compile_variable_assignment(&mut self, assign: &Assignment) -> Result<()> {
        let new_value = self.compile_expression(&assign.value)?;
        match &assign.target {
            crate::ast::Expression::Variable(var) => {
                let ptr = self.variables.get(var)
                                .ok_or_else(|| {
                                    anyhow!("Compiler Bug: Variable '{}' not found in symbol table during assignment compilation.", var)
                                })?;

                self.builder
                    .build_store(ptr.1, new_value)
                    .map_err(|e| anyhow!("Failed to store new value in '{}': {}", var, e))?;

                Ok(())
            }
            crate::ast::Expression::ArrayAccess(var) => {
                let array_name = &var.array_name;
                let index_value = self.compile_expression(&var.index)?;
                let (array_type, array_ptr) = self
                    .variables
                    .get(array_name)
                    .ok_or_else(|| anyhow!("Array variable '{}' not found", array_name))?;

                let index_value = match index_value {
                    BasicValueEnum::IntValue(val) => val,
                    _ => return Err(anyhow!("Array index must be an integer")),
                };
                let zero = self.context.i32_type().const_zero();
                let indices = [zero, index_value];
                let element_ptr = unsafe {
                    self.builder.build_gep(
                        *array_type,
                        *array_ptr,
                        &indices,
                        &format!("array_index_{}", array_name),
                    )
                }?;
                let element_type = match array_type {
                    BasicTypeEnum::ArrayType(array_ty) => array_ty.get_element_type(),
                    _ => {
                        return Err(anyhow!(
                            "Expected an array type for variable '{}'",
                            array_name
                        ));
                    }
                };
                self.builder
                    .build_store(element_ptr, new_value)
                    .map_err(|e| {
                        anyhow!("Failed to store new value in array '{}': {}", array_name, e)
                    })?;
                self.last_value = Some(new_value);
                self.builder.build_load(
                    element_type,
                    element_ptr,
                    &format!("array_element_{}", array_name),
                )?;

                Ok(())
            }
            crate::ast::Expression::StructFieldAccess(var) => {
                let struct_name =
                    if let crate::ast::Expression::Variable(var_name) = &*var.struct_expr {
                        var_name
                    } else {
                        return Err(anyhow!("Invalid struct expression"));
                    };

                let field_name = &var.field_name;

                let (struct_type, struct_ptr) = self
                    .variables
                    .get(struct_name)
                    .ok_or_else(|| anyhow!("Struct variable '{:?}' not found", struct_name))?;

                let struct_type = if let BasicTypeEnum::StructType(ty) = struct_type {
                    *ty
                } else {
                    return Err(anyhow!(
                        "Expected struct type for variable '{}'",
                        struct_name
                    ));
                };

                let field_index =
                    self.get_field_index(struct_type.get_name().unwrap().to_str()?, field_name)?;

                let field_type = struct_type;

                let field_ptr = self.builder.build_struct_gep(
                    struct_type,
                    *struct_ptr,
                    field_index as u32,
                    &format!("{}_field_ptr_{}", struct_name, field_name),
                )?;

                self.builder
                    .build_store(field_ptr, new_value)
                    .map_err(|e| {
                        anyhow!(
                            "Failed to store new value in struct field '{}': {}",
                            field_name,
                            e
                        )
                    })?;

                self.last_value = Some(new_value);
                self.builder.build_load(
                    field_type,
                    field_ptr,
                    &format!("{}_field_val_{}", struct_name, field_name),
                )?;

                Ok(())
            }

            _ => Err(anyhow!(
                "Invalid assignment target: expected a variable, found {:?}",
                assign.target
            )),
        }
    }
}
