use super::CodeGenerator;
use crate::ast::Literal;
use anyhow::Result;
use inkwell::{AddressSpace, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_literal(&mut self, literal: &Literal) -> Result<BasicValueEnum<'ctx>> {
        Ok(match literal {
            Literal::Int(value) => {
                let u64_type = self.context.i64_type();
                let u64_value = u64_type.const_int(*value, false);
                BasicValueEnum::IntValue(u64_value)
            }
            Literal::Float(value) => {
                let f64_type = self.context.f64_type();
                let f64_value = f64_type.const_float(*value);
                BasicValueEnum::FloatValue(f64_value)
            }
            Literal::Bool(value) => {
                let bool_type = self.context.bool_type();
                let bool_value = bool_type.const_int(if *value { 1 } else { 0 }, false);
                BasicValueEnum::IntValue(bool_value)
            }
            Literal::String(value) => {
                let string_len = value.len() as u64;
                let total_size = 8 + string_len + 1; // length field + string data + null terminator

                // Allocate memory
                let raw_ptr = self
                    .builder
                    .build_call(
                        self.module.get_function("malloc").unwrap(),
                        &[self.context.i64_type().const_int(total_size, false).into()],
                        "malloc_string",
                    )?
                    .try_as_basic_value()
                    .left()
                    .unwrap()
                    .into_pointer_value();

                // Define the string struct type
                let string_struct_type = self.context.struct_type(
                    &[
                        self.context.i64_type().into(),              // length
                        self.context.i8_type().array_type(0).into(), // flexible array for data
                    ],
                    false,
                );

                // Cast raw pointer to struct pointer
                let string_struct_ptr = self.builder.build_pointer_cast(
                    raw_ptr,
                    self.context.ptr_type(AddressSpace::default()),
                    "string_struct_ptr",
                )?;

                // Store the length in the first field
                let length_ptr = self.builder.build_struct_gep(
                    string_struct_type,
                    string_struct_ptr,
                    0,
                    "length_ptr",
                )?;
                let length_value = self.context.i64_type().const_int(string_len, false);
                self.builder.build_store(length_ptr, length_value)?;

                // Get pointer to the data field (flexible array)
                let data_ptr = self.builder.build_struct_gep(
                    string_struct_type,
                    string_struct_ptr,
                    1,
                    "data_ptr",
                )?;

                // Copy the string data
                let string_global = self.builder.build_global_string_ptr(value, "temp_string")?;
                let memcpy_fn = self.module.get_function("llvm.memcpy.p0.p0.i64").unwrap();

                self.builder.build_call(
                    memcpy_fn,
                    &[
                        data_ptr.into(),                         // dest
                        string_global.as_pointer_value().into(), // src
                        self.context
                            .i64_type()
                            .const_int(string_len + 1, false)
                            .into(), // size (include null terminator)
                        self.context.bool_type().const_zero().into(), // volatile = false
                    ],
                    "memcpy_string",
                )?;

                string_struct_ptr.into()
            }
            Literal::Char(value) => {
                let char_type = self.context.i32_type();
                let char_value = char_type.const_int(*value as u64, false);
                BasicValueEnum::IntValue(char_value)
            }
        })
    }
}
