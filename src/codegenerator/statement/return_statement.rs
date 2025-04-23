use super::CodeGenerator;
use crate::ast::ReturnStatement;
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_ret_stmt(&mut self, ret_stmt: &ReturnStatement) -> Result<()> {
        match &ret_stmt.value {
            None => {
                self.builder.build_return(None)?;
            }
            Some(expr) => {
                // First, compile the expression to get the LLVM value to return.
                let compiled_value = self.compile_expression(expr)?;
                self.builder.build_return(Some(&compiled_value))?;
            }
        }
        Ok(())
    }
}
