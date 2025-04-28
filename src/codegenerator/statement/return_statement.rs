use super::CodeGenerator;
use crate::ast::ReturnStatement;
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_ret_stmt(&mut self, ret_stmt: &ReturnStatement) -> Result<()> {
        if let Some(expr) = &ret_stmt.value {
            // If there's a value to return, compile the expression
            let return_value = self.compile_expression(expr)?;
            self.builder.build_return(Some(&return_value));
        } else {
            // No return value (for void functions)
            self.builder.build_return(None);
        }
        
        Ok(())
    }
}
