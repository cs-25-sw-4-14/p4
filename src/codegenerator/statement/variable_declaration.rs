use super::CodeGenerator;
use crate::ast::VariableDeclaration;
use anyhow::{Result, anyhow};

impl<'ctx> CodeGenerator<'ctx> {
    fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        Ok(())
    }
}
