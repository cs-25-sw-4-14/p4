use crate::{ast::FunctionCall, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::values::{BasicMetadataValueEnum, CallSiteValue, FunctionValue};

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a function call (e.g., `foo(1, 2)`).
    /// Returns the `CallSiteValue`, which represents the LLVM-level function call result.
    pub fn compile_function_call_stmt(
        &mut self,
        func_call: &FunctionCall,
    ) -> Result<CallSiteValue<'ctx>> {
        let function_name = &func_call.name;

        // 🔍 Look up the function by name in the current LLVM module.
        // This assumes the function has already been declared or defined.
        let function: FunctionValue<'ctx> =
            self.module.get_function(function_name).ok_or_else(|| {
                // If not found, we throw an internal error since this shouldn't happen in a well-formed program.
                anyhow!(
                    "Compiler Bug: Function '{}' not found in LLVM module during call compilation.",
                    function_name
                )
            })?;

        // 📦 Prepare a vector to hold the compiled argument values.
        // Reserve capacity equal to the number of arguments.
        let mut compiled_args: Vec<BasicMetadataValueEnum<'ctx>> =
            Vec::with_capacity(func_call.args.len());

        // 🔁 Compile each argument expression in order and convert them
        // to `BasicMetadataValueEnum`, which is required by the `build_call` method.
        for arg_expr in &func_call.args {
            let arg_value = self.compile_expression(arg_expr)?;
            compiled_args.push(arg_value.into());
        }

        // ⚙️ Generate the LLVM IR `call` instruction.
        // This inserts a call to the function with the compiled arguments.
        let call_site = self
            .builder
            .build_call(function, &compiled_args, function_name)
            .map_err(|e| anyhow!("Failed to build function call '{}': {}", function_name, e))?;

        // ✅ Return the CallSiteValue, which represents the LLVM call instruction.
        Ok(call_site)
    }
}
