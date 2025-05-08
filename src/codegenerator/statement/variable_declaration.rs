use super::CodeGenerator;
use crate::ast::VariableDeclaration;
use anyhow::{Result, anyhow};

impl CodeGenerator<'_> {
    pub fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        let value = self.compile_expression(&decl.value)?;

        let value_type = value.get_type();
        let var_name = &decl.name;

        let array_ptr = self
            .builder
            .build_alloca(value_type, var_name)
            .map_err(|e| {
                anyhow!(
                    "Failed to allocate memory for variable '{}': {}",
                    var_name,
                    e
                )
            })?;

        self.builder.build_store(array_ptr, value)?;
        self.variables
            .insert(var_name.clone(), (value_type, array_ptr));

        Ok(())
    }
}
