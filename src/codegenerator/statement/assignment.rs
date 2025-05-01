use crate::{ast::Assignment, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_variable_assignment(&mut self, assign: &Assignment) -> Result<()> {
        let new_value = self.compile_expression(&assign.value)?;
        match &assign.target {
            crate::ast::Expression::Variable(var) => {
                let ptr = self.variables.get(var)
                                .ok_or_else(|| {
                                    anyhow!("Compiler Bug: Variable '{}' not found in symbol table during assignment compilation.", var)
                                })?;

                self.builder
                    .build_store(ptr.1, new_value)
                    .map_err(|e| anyhow!("Failed to store new value in '{}': {}", var, e))?;

                Ok(())
            }

            _ => {
                return Err(anyhow!(
                    "Invalid assignment target: expected a variable, found {:?}",
                    assign.target
                ));
            }
        }
    }
}
