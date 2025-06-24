use super::CodeGenerator;
use crate::ast::FunctionCall;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a function call expression, returning the result value.
    ///
    /// # Arguments
    /// * `function_call` - The AST node representing the function call, including name and arguments.
    ///
    /// # Returns
    /// * `Result<BasicValueEnum>` - The LLVM value representing the function call result, or an error.
    pub fn compile_function_call_expr(
        &mut self,
        function_call: &FunctionCall,
    ) -> Result<BasicValueEnum<'ctx>> {
        // Look up the function in the LLVM module by name.
        let function = match self.module.get_function(&function_call.name) {
            Some(f) => f,
            None => {
                // Return an error if the function is not found.
                return Err(anyhow::anyhow!(
                    "Function '{}' not found",
                    function_call.name
                ));
            }
        };

        // Compile each argument expression recursively into LLVM values.
        let mut arg_values = Vec::with_capacity(function_call.args.len());
        for arg_expr in &function_call.args {
            arg_values.push(self.compile_expression(arg_expr)?);
        }

        // Convert each argument value into LLVM's BasicMetadataValueEnum as required for calls.
        let args: Vec<inkwell::values::BasicMetadataValueEnum> =
            arg_values.iter().map(|val| (*val).into()).collect();

        // Build the LLVM call instruction with the function and prepared arguments.
        let call_site_value = self
            .builder
            .build_call(function, &args, &format!("call_{}", function_call.name))?
            // Try to extract the returned LLVM value from the call instruction.
            .try_as_basic_value()
            .left();

        // Return the call result or an error if the function has void return type (no value).
        call_site_value.ok_or_else(|| anyhow::anyhow!("Function call did not return a value"))
    }
}
