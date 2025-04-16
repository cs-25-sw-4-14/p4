use crate::ast::StructDef;
use super::CodeGenerator;
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct(&self, struct_def: &StructDef) -> Result<()> {
        todo!()
    }
}
