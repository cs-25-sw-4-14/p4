use super::CodeGenerator;
use crate::ast::Literal;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a literal value (int, float, bool, string, char) into an LLVM value.
    pub fn compile_literal(&mut self, literal: &Literal) -> Result<BasicValueEnum<'ctx>> {
        Ok(match literal {
            // Integer literal: create a 64-bit integer constant
            Literal::Int(value) => {
                let u64_type = self.context.i64_type();
                let u64_value = u64_type.const_int(*value, false);
                BasicValueEnum::IntValue(u64_value)
            }

            // Floating-point literal: create a 64-bit float constant
            Literal::Float(value) => {
                let f64_type = self.context.f64_type();
                let f64_value = f64_type.const_float(*value);
                BasicValueEnum::FloatValue(f64_value)
            }

            // Boolean literal: create a 1-bit integer (LLVM bool) constant (1 or 0)
            Literal::Bool(value) => {
                let bool_type = self.context.bool_type();
                let bool_value = bool_type.const_int(if *value { 1 } else { 0 }, false);
                BasicValueEnum::IntValue(bool_value)
            }

            // String literal: allocate memory and copy string data with length prefix
            Literal::String(value) => {
                let string_len = value.len() as u64;
                let total_size = 8 + string_len + 1; // 8 bytes for length + string bytes + null terminator

                // Call malloc to allocate memory for the string structure + data
                let string_ptr = self
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

                // Store the length at the start of allocated memory (first 8 bytes)
                let length_value = self.context.i64_type().const_int(string_len, false);
                self.builder.build_store(string_ptr, length_value)?;

                // Compute pointer to the string data part (after the length prefix)
                let data_ptr = unsafe {
                    self.builder.build_gep(
                        self.context.i8_type(),          // pointer to i8 (byte)
                        string_ptr,                     // base pointer (to length)
                        &[self.context.i64_type().const_int(8, false)], // offset by 8 bytes
                        "data_ptr",
                    )?
                };

                // Create a global constant for the string content (null terminated)
                let string_global = self.builder.build_global_string_ptr(value, "temp_string")?;

                // Call LLVM's memcpy intrinsic to copy string content into allocated memory
                let memcpy_fn = self.module.get_function("llvm.memcpy.p0.p0.i64").unwrap();

                self.builder.build_call(
                    memcpy_fn,
                    &[
                        data_ptr.into(),                         // destination pointer
                        string_global.as_pointer_value().into(), // source pointer
                        self.context
                            .i64_type()
                            .const_int(string_len + 1, false) // size including null terminator
                            .into(),
                        self.context.bool_type().const_zero().into(), // volatile = false
                    ],
                    "memcpy_string",
                )?;

                // Return the pointer to the allocated string structure
                string_ptr.into()
            }

            // Char literal: represented as a 32-bit integer value
            Literal::Char(value) => {
                let char_type = self.context.i32_type();
                let char_value = char_type.const_int(*value as u64, false);
                BasicValueEnum::IntValue(char_value)
            }
        })
    }
}
