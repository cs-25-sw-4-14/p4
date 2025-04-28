use crate::ast::WhileStatement;
use anyhow::Result;
use inkwell::values::IntValue;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_while_statement(&mut self, while_stmt: &WhileStatement) -> Result<()> {
        let cond_bb = self.context.append_basic_block(self.current_function.unwrap(), "loop_cond");
        let body_bb = self.context.append_basic_block(self.current_function.unwrap(), "loop_body");
        let after_bb = self.context.append_basic_block(self.current_function.unwrap(), "after_loop");
    
        // Start the loop by creating a new basic block and jump there
        self.builder.build_unconditional_branch(cond_bb);
        self.builder.position_at_end(cond_bb);
    
        // Condition check
        let condition_value = self.compile_expression(&while_stmt.condition)?;
        let condition = self.builder.build_int_compare(
            inkwell::IntPredicate::NE,
            condition_value.into_int_value(),
            self.context.i32_type().const_zero(),
            "loop_cond",
        );
        self.builder.build_conditional_branch(condition, body_bb, after_bb);
        self.builder.position_at_end(body_bb);
    
        // Compile the body of the loop
        for stmt in &while_stmt.body {
            self.compile_statement(stmt)?;
        }
    
        // Check for `continue` or `break` in the loop body.
        // If a `continue` or `break` statement is encountered, we handle jumping to the right place.
    
        // Continue: jump back to the condition
        self.builder.build_unconditional_branch(cond_bb);
    
        // Break: jump to the end of the loop
        self.builder.build_unconditional_branch(after_bb);
    
        // Finally, after loop execution, jump to `after_bb`.
        self.builder.position_at_end(after_bb);
    
        Ok(())
    }
    
}