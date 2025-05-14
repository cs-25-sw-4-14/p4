use super::CodeGenerator;

use crate::ast::Expression;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_reference(&mut self, expr: &Expression, _mutable: &bool) -> Result<BasicValueEnum<'ctx>> {
        let value = self.compile_expression(expr)?;
        let value_type = value.get_type();

        let ptr = self.builder.build_alloca(value_type, "ref_tmp")?;

        _ = self.builder.build_store(ptr, value);
        Ok(ptr.into())
    }
}
