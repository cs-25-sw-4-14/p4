use crate::{ast::WhileStatement, codegenerator::CodeGenerator};
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_while_statement(&mut self, while_stmt: &WhileStatement) -> Result<()> {
        // Get parent function
        let parent_fn = self.builder
            .get_insert_block()
            .and_then(|bb| bb.get_parent())
            .ok_or_else(|| anyhow::anyhow!("Not inside a function"))?;
            
        // Create basic blocks for the loop
        let cond_bb = self.context.append_basic_block(parent_fn, "loop_cond");
        let body_bb = self.context.append_basic_block(parent_fn, "loop_body");
        let after_bb = self.context.append_basic_block(parent_fn, "after_loop");

        // Jump to the condition block
        self.builder.build_unconditional_branch(cond_bb);
        self.builder.position_at_end(cond_bb);

        // Compile the condition
        let condition_value = self.compile_expression(&while_stmt.condition)?;
        let condition = match condition_value {
            inkwell::values::BasicValueEnum::IntValue(int_val) => int_val,
            _ => return Err(anyhow::anyhow!("While condition must be a boolean")),
        };

        // Branch based on condition
        self.builder.build_conditional_branch(condition, body_bb, after_bb);
        
        // Compile the loop body
        self.builder.position_at_end(body_bb);
        
        // Enter a new scope for the loop body
        self.enter_scope();
        
        // Compile each statement in the body
        for stmt in &while_stmt.body {
            self.compile_statement(stmt)?;
        }
        
        // Clean up the scope when we exit the body
        self.exit_scope()?;
        
        // Jump back to the condition if the body didn't end with a terminator
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(cond_bb);
        }
        
        // Continue from the after block
        self.builder.position_at_end(after_bb);
        
        Ok(())
    }
}
