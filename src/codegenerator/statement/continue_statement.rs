use super::CodeGenerator;
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_continue(&mut self) -> Result<()> {
        if let Some(cond_block) = self.loop_cond_block {
            self.builder.build_unconditional_branch(cond_block);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Continue outside of loop"))
        }
    }
}
