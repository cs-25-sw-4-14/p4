use super::CodeGenerator;
use crate::ast::VariableDeclaration;
use anyhow::{Result, anyhow};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        let value = self.compile_expression(&decl.value)?;

        let ptr = self
            .builder
            .build_alloca(self.get_basic_type(&decl.var_type), &decl.name)
            .map_err(|e| {
                anyhow!(
                    "Failed to allocate memory for variable '{}': {}",
                    decl.name,
                    e
                )
            })?;

        self.builder
            .build_store(ptr, value)
            .map_err(|e| anyhow!("Failed to store value in variable '{}': {}", decl.name, e))?;

        let llvm_type = self.get_basic_type(&decl.var_type);

        // Registers the variable
        self.variables.insert(decl.name.clone(), (llvm_type, ptr));

        Ok(())
    }
}
