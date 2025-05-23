use crate::{ast::VariableAssignment, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};

impl CodeGenerator<'_> {
    pub fn compile_variable_assignment(&mut self, assign: &VariableAssignment) -> Result<()> {
        if assign.target == "_" {
            return Ok(());
        }

        let new_value = self.compile_expression(&assign.value)?;
        let ptr = self.variables.get(&assign.target)
                                .ok_or_else(|| {
                                    anyhow!("Compiler Bug: Variable '{}' not found in symbol table during assignment compilation.", assign.target)
                                })?;

        self.builder
            .build_store(ptr.1, new_value)
            .map_err(|e| anyhow!("Failed to store new value in '{}': {}", assign.target, e))?;

        Ok(())
    }
}
