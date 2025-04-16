use super::CodeGenerator;
use anyhow::{Result, anyhow};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_variable(&mut self, name: &String) -> Result<BasicValueEnum<'ctx>> {
        let (var_type, var_ptr) = self.variables.get(name)
            .ok_or_else(|| anyhow!("Undefined variable: {}", name))?;
        let loaded = self.builder.build_load(*var_type, *var_ptr, name);
        Ok(loaded?)
    }
}
