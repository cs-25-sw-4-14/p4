use super::CodeGenerator;
use crate::ast::Expression;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_dereference(&mut self, expr: &Expression) -> Result<BasicValueEnum<'ctx>> {
        let ptr_value = self.compile_expression(expr)?;

        let ptr = match ptr_value {
            BasicValueEnum::PointerValue(ptr) => ptr,
            _ => anyhow::bail!("Cannot dereference a non-pointer value"),
        };

        Ok(ptr.into())

    }
}
