use super::CodeGenerator;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_variable(&mut self, _name: &String) -> Result<BasicValueEnum<'ctx>> {
        todo!()
        // let (var_type, var_ptr) = self
        //     .variables
        //     .get(name)
        //     .ok_or_else(|| anyhow!("Undefined variable: {}", name))?;
        // let loaded = self.builder.build_load(*var_type, *var_ptr, name);
        // Ok(loaded?)
    }
}
