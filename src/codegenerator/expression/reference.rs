use super::CodeGenerator;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_reference(&mut self, expr: &String) -> Result<BasicValueEnum<'ctx>> {
        Ok(self.compile_variable(expr)?)
    }
}
