use super::CodeGenerator;
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_break(&mut self) -> Result<()> {
        if let Some(end_block) = self.loop_end_block {
            self.builder.build_unconditional_branch(end_block);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Break outside of loop"))
        }
    }
}
