use super::CodeGenerator;
use crate::ast::StructDef;
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct(&self, _struct_def: &StructDef) -> Result<()> {
        todo!()
    }
}
