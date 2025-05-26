use super::CodeGenerator;
use anyhow::{Result, anyhow};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_break(&mut self) -> Result<()> {
        // Look at the top of the break‐target stack
        if let Some(&end_block) = self.break_target_stack.last() {
            // Unconditionally branch to the loop’s end block
            _ = self.builder.build_unconditional_branch(end_block);
            Ok(())
        } else {
            Err(anyhow!("`break` outside of loop"))
        }
    }
}
