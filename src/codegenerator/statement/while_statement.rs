use crate::{ast::WhileStatement, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::IntPredicate;

impl CodeGenerator<'_> {
    /// Compiles a `while` loop statement.
    ///
    /// Example:
    /// ```rust
    /// while (condition) {
    ///     // body
    /// }
    /// ```
    pub fn compile_while_statement(&mut self, while_stmt: &WhileStatement) -> Result<()> {
        // Get the current LLVM function we're inserting code into.
        // This must exist, otherwise we can't create new blocks.
        let current_fn = match self.current_function {
            Some(function) => function,
            None => return Err(anyhow!("No active function context for while statement")),
        };

        // Create a new basic block for the loop condition check
        let cond_bb = self.context.append_basic_block(current_fn, "loop_cond");

        // Create a new basic block for the loop body
        let body_bb = self.context.append_basic_block(current_fn, "loop_body");

        // Create a new basic block for code after the loop (loop exit)
        let after_bb = self.context.append_basic_block(current_fn, "after_loop");

        // Push the loop exit block to the break stack so 'break' can jump here
        self.break_target_stack.push(after_bb);

        // Push the condition block to the continue stack so 'continue' jumps here
        self.continue_target_stack.push(cond_bb);

        // Unconditionally jump to the condition block to start the loop
        _ = self.builder.build_unconditional_branch(cond_bb);

        // Set insertion point to the condition block to generate condition code
        self.builder.position_at_end(cond_bb);

        // Compile the loop condition expression (e.g. `while_stmt.condition`)
        let condition_value = self.compile_expression(&while_stmt.condition)?;

        // Generate LLVM IR for the loop condition depending on type
        let condition = match condition_value {
            // For integer conditions, compare against zero (not equal)
            inkwell::values::BasicValueEnum::IntValue(int_val) => {
                let zero = int_val.get_type().const_zero();
                self.builder
                    .build_int_compare(IntPredicate::NE, int_val, zero, "loop_cond")
            }
            // For floating point conditions, compare if not equal to zero
            inkwell::values::BasicValueEnum::FloatValue(float_val) => {
                let zero = float_val.get_type().const_zero();
                self.builder.build_float_compare(
                    inkwell::FloatPredicate::ONE,
                    float_val,
                    zero,
                    "loop_cond",
                )
            }
            // If condition is neither int nor float, return an error
            _ => return Err(anyhow!("Unsupported condition type in while statement")),
        };

        // Build conditional branch based on condition:
        // If true, jump to loop body; else jump to after loop block
        _ = self
            .builder
            .build_conditional_branch(condition.unwrap(), body_bb, after_bb);

        // Position builder at the start of the loop body block
        self.builder.position_at_end(body_bb);

        // Compile each statement in the loop body
        for stmt in &while_stmt.body {
            self.compile_statement(stmt)?;
        }

        // After compiling body, check if the current block has a terminator (like return or branch)
        let current_block = self.builder.get_insert_block().unwrap();
        if current_block.get_terminator().is_none() {
            // If no terminator, create an unconditional branch back to the condition block to loop
            _ = self.builder.build_unconditional_branch(cond_bb);
        }

        // Pop the break and continue targets for this loop context
        self.break_target_stack.pop();
        self.continue_target_stack.pop();

        // Position builder at the after-loop block for subsequent code generation
        self.builder.position_at_end(after_bb);

        Ok(())
    }
}
