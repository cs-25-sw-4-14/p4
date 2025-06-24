use crate::{ast::VariableAssignment, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};

impl CodeGenerator<'_> {
    /// Compiles an assignment to a variable.
    ///
    /// Example: `x = expr;`
    pub fn compile_variable_assignment(&mut self, assign: &VariableAssignment) -> Result<()> {
        // Ignore assignments to the special discard variable "_"
        // (like `_ = expr;`), which means "evaluate expr but don't store".
        if assign.target == "_" {
            return Ok(());
        }

        // Compile the expression on the right-hand side of the assignment
        let new_value = self.compile_expression(&assign.value)?;

        // Look up the pointer to the target variable in the symbol table.
        // `self.variables` maps variable names to (type, pointer) tuples.
        let ptr = self.variables.get(&assign.target)
            .ok_or_else(|| {
                // If variable not found, this is a compiler bug.
                anyhow!("Compiler Bug: Variable '{}' not found in symbol table during assignment compilation.", assign.target)
            })?;

        // Emit an LLVM 'store' instruction to write the new value into the variable's memory.
        self.builder
            .build_store(ptr.1, new_value)
            .map_err(|e| anyhow!("Failed to store new value in '{}': {}", assign.target, e))?;

        Ok(())
    }
}
