use crate::ast::{Expression, FunctionCall, Statement}; 
use anyhow::{Result, anyhow};
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, CallSiteValue, FunctionValue}; 

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_function_call(&mut self,func_call: &FunctionCall) -> Result<CallSiteValue<'ctx>> {
        let function_name = &func_call.name;

        // 1. Look up the function in the LLVM module.
        let function: FunctionValue<'ctx> = self.module.get_function(function_name)
            .ok_or_else(|| {
                anyhow!("Compiler Bug: Function '{}' not found in LLVM module during call compilation.", function_name)
            })?;

        // 2. Compile all argument expressions.
        let mut compiled_args: Vec<BasicMetadataValueEnum<'ctx>> = Vec::with_capacity(func_call.args.len());
        for arg_expr in &func_call.args {
            // Recursively compile each argument expression.
            let arg_value = self.compile_expression(arg_expr)?;
            compiled_args.push(arg_value.into()); // Convert BasicValueEnum into BasicMetadataValueEnum for the call instruction
        }

        // 3. Build the LLVM 'call' instruction.
        let call_site = self.builder
        .build_call(function, &compiled_args, &(function_name.to_string() + "_call_ret"))
        .map_err(|e| anyhow!("Failed to build function call '{}': {}", function_name, e))?;
    
        Ok(call_site)

    }
}