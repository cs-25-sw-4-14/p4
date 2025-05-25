use super::CodeGenerator;
use anyhow::Result;

impl CodeGenerator<'_> {
    pub fn compile_continue(&mut self) -> Result<()> {
        if let Some(&cond_block) = self.continue_target_stack.last() {
            _ = self.builder.build_unconditional_branch(cond_block);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Continue outside of loop"))
        }
    }
}
