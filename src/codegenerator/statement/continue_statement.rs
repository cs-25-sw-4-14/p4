use super::CodeGenerator;
use anyhow::Result;

impl CodeGenerator<'_> {
    /// Compiles a `continue` statement.
    ///
    /// In control flow terms, `continue` means: jump to the beginning of the loop (condition check).
    /// This assumes there is a loop context, and that the target condition block is stored on a stack.
    pub fn compile_continue(&mut self) -> Result<()> {
        // Try to get the top value from the continue target stack.
        // This should be the basic block that represents the loop's condition (or beginning).
        if let Some(&cond_block) = self.continue_target_stack.last() {
            // Emit an unconditional branch to that block.
            // This causes execution to jump to the loop condition, effectively "continuing" the loop.
            _ = self.builder.build_unconditional_branch(cond_block);

            // Return success; no value is produced.
            Ok(())
        } else {
            // If there is no target block on the stack, we're not inside a loop.
            // Return an error because `continue` is invalid here.
            Err(anyhow::anyhow!("Continue outside of loop"))
        }
    }
}
