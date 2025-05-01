use super::CodeGenerator;
use anyhow::Result;

impl CodeGenerator<'_> {
    pub fn compile_break(&mut self) -> Result<()> {
        todo!();
        // if let Some(end_block) = self.loop_end_block {
        //     self.builder.build_unconditional_branch(end_block);
        //     Ok(())
        // } else {
        //     Err(anyhow::anyhow!("Break outside of loop"))
        // }
    }
}
