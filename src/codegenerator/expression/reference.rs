use super::CodeGenerator;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a reference to a variable given its name as a string.
    ///
    /// # Arguments
    /// * `expr` - The name of the variable to reference.
    ///
    /// # Returns
    /// Returns a `BasicValueEnum` representing the value of the variable.
    ///
    /// # Errors
    /// Returns an error if the variable with the given name cannot be found or compiled.
    pub fn compile_reference(&mut self, expr: &String) -> Result<BasicValueEnum<'ctx>> {
        // Delegates to compile_variable to get the LLVM value of the variable.
        Ok(self.compile_variable(expr)?)
    }
}
