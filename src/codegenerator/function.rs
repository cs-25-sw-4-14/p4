use anyhow::{anyhow, Result};
use inkwell::values::FunctionValue;

use crate::ast::FunctionDef;
use super::CodeGenerator;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_function(&mut self, func: &FunctionDef) -> Result<()> {
        // TODO: Implement parameters
        
        // Compile the function body.
        for stmt in &func.body {
            self.compile_statement(stmt)?;
        }

        // TODO: Return Function value
        Ok(())
    }
}
