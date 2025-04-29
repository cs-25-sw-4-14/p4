use crate::{ast::VariableAssignment, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_variable_assignment(&mut self, assign: &VariableAssignment) -> Result<()> {
        let new_value = self.compile_expression(&assign.value)?;
        let variable_name = &assign.target;

        let ptr = self.variables.get(variable_name)
            .copied() // Get the pointer value for the variable from the symbol table
            .ok_or_else(|| {
                anyhow!("Compiler Bug: Variable '{}' not found in symbol table during assignment compilation.", variable_name)
            })?;

        self.builder
            .build_store(ptr, new_value)
            .map_err(|e| anyhow!("Failed to store new value in '{}': {}", variable_name, e))?;

        Ok(())
    }
}

