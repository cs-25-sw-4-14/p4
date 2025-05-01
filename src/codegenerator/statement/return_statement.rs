use super::CodeGenerator;
use crate::ast::ReturnStatement;
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_ret_stmt(&mut self, ret_stmt: &ReturnStatement) -> Result<()> {
        // Before returning, clean up all local variables in scope
        // We need to clear all scopes up to the function scope
        while self.scope_stack.len() > 1 {
            self.exit_scope()?;
        }
        
        if let Some(expr) = &ret_stmt.value {
            // If there's a value to return, compile the expression
            let return_value = self.compile_expression(expr)?;
            
            // Build the return instruction
            _ = self.builder.build_return(Some(&return_value));
        } else {
            // No return value (for void functions)
            _ = self.builder.build_return(None);
        }
        
        Ok(())
    }
}
