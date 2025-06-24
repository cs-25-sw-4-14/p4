use crate::{ast::Debug, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::values::BasicValueEnum;

impl CodeGenerator<'_> {
    /// Compiles a debug print statement into a call to `printf`.
    ///
    /// It supports integers, floats, and strings (via pointers).
    pub fn compile_print_stmt(&mut self, stmt: &Debug) -> Result<()> {
        // 👇 Compile the expression whose value we want to print
        let value = self.compile_expression(&stmt.expression)?;

        // 🔍 Look up the `printf` function in the current module
        // This assumes it's declared already (likely as an external declaration)
        let printf = self
            .module
            .get_function("printf")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;

        // 🧠 Depending on the type of value, choose the right format string
        // and prepare the arguments to pass to printf
        let call_args = match value {
            // === Integer Values ===
            BasicValueEnum::IntValue(int_val) => match int_val.get_type().get_bit_width() {
                1 => {
                    // Boolean (1-bit int): use "%d"
                    let fmt_str = self.builder.build_global_string_ptr("%d", "fmt_bool")?;
                    vec![fmt_str.as_pointer_value().into(), int_val.into()]
                }
                8 => {
                    // 8-bit int: use "%hhi"
                    let fmt_str = self.builder.build_global_string_ptr("%hhi", "fmt_i8")?;
                    vec![fmt_str.as_pointer_value().into(), int_val.into()]
                }
                16 => {
                    // 16-bit int: use "%hi"
                    let fmt_str = self.builder.build_global_string_ptr("%hi", "fmt_i16")?;
                    vec![fmt_str.as_pointer_value().into(), int_val.into()]
                }
                32 => {
                    // 32-bit int: use "%d"
                    let fmt_str = self.builder.build_global_string_ptr("%d", "fmt_i32")?;
                    vec![fmt_str.as_pointer_value().into(), int_val.into()]
                }
                64 => {
                    // 64-bit int: use "%lld"
                    let fmt_str = self.builder.build_global_string_ptr("%lld", "fmt_i64")?;
                    vec![fmt_str.as_pointer_value().into(), int_val.into()]
                }
                other => {
                    // ❌ Unknown or unsupported bit-width
                    return Err(anyhow!("Unsupported integer bit width: {}", other));
                }
            },

            // === Float Values ===
            BasicValueEnum::FloatValue(float_val) => {
                match self.get_float_bit_width(float_val.get_type()) {
                    Ok(16) => {
                        // 16-bit float: LLVM doesn't support printing directly
                        // So extend it to 32-bit before passing to printf
                        let fmt_str = self.builder.build_global_string_ptr("%.10f", "fmt_f16")?;
                        let extended = self.builder.build_float_ext(
                            float_val,
                            self.context.f32_type(),
                            "f16_to_f32",
                        );
                        vec![fmt_str.as_pointer_value().into(), extended.unwrap().into()]
                    }
                    Ok(32) => {
                        // 32-bit float: use "%.10f"
                        let fmt_str = self.builder.build_global_string_ptr("%.10f", "fmt_f32")?;
                        vec![fmt_str.as_pointer_value().into(), float_val.into()]
                    }
                    Ok(64) => {
                        // 64-bit float: use "%.10lf"
                        let fmt_str = self.builder.build_global_string_ptr("%.10lf", "fmt_f64")?;
                        vec![fmt_str.as_pointer_value().into(), float_val.into()]
                    }
                    Err(e) => {
                        return Err(anyhow!("Failed to get float bit width: {}", e));
                    }
                    _ => {
                        return Err(anyhow!("Unsupported float type"));
                    }
                }
            }

            // === Pointer Values ===
            BasicValueEnum::PointerValue(ptr_val) => {
                if ptr_val.is_null() {
                    // Handle null pointer: just print a newline
                    let fmt_str = self.builder.build_global_string_ptr("\n", "fmt")?;
                    vec![fmt_str.as_pointer_value().into(), ptr_val.into()]
                } else {
                    // Otherwise, assume it's a C-style string (char pointer)
                    let data_ptr = self.get_string_data_ptr(ptr_val)?; // Get inner string pointer
                    let fmt_str = self.builder.build_global_string_ptr("%s", "fmt")?;
                    vec![fmt_str.as_pointer_value().into(), data_ptr.into()]
                }
            }

            // ❌ All other types are unsupported for debug printing
            _ => return Err(anyhow!("not a valid type for debug statement")),
        };

        // 🛠 Emit the actual `call` to printf with the prepared arguments
        let _ = self.builder.build_call(printf, &call_args, "printf_call");

        // ✅ We're done—return success
        Ok(())
    }
}
