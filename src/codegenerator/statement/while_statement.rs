use crate::{ast::WhileStatement, codegenerator::CodeGenerator};
use anyhow::Result;
impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_while_statement(&mut self, while_stmt: &WhileStatement) -> Result<()> {
        //     let cond_bb = self
        //         .context
        //         .append_basic_block(self.current_function.unwrap(), "loop_cond");
        //     let body_bb = self
        //         .context
        //         .append_basic_block(self.current_function.unwrap(), "loop_body");
        //     let after_bb = self
        //         .context
        //         .append_basic_block(self.current_function.unwrap(), "after_loop");
        //
        //     // Push loop control targets
        //     self.break_target_stack.push(after_bb);
        //     self.continue_target_stack.push(cond_bb);
        //
        //     // Start loop: jump to condition check
        //     self.builder.build_unconditional_branch(cond_bb);
        //     self.builder.position_at_end(cond_bb);
        //
        //     // Compile loop condition
        //     let condition_value = self.compile_expression(&while_stmt.condition)?;
        //     let condition = self.builder.build_int_compare(
        //         inkwell::IntPredicate::NE,
        //         condition_value.into_int_value(),
        //         self.context.i32_type().const_zero(),
        //         "loop_cond",
        //     );
        //
        //     // Branch based on condition
        //     self.builder
        //         .build_conditional_branch(condition, body_bb, after_bb);
        //     self.builder.position_at_end(body_bb);
        //
        //     // Compile loop body
        //     for stmt in &while_stmt.body {
        //         self.compile_statement(stmt)?;
        //     }
        //
        //     // Check if the body ended with a terminator (e.g., return, break)
        //     // If not, jump back to loop condition
        //     if self
        //         .builder
        //         .get_insert_block()
        //         .unwrap()
        //         .get_terminator()
        //         .is_none()
        //     {
        //         self.builder.build_unconditional_branch(cond_bb);
        //     }
        //
        //     // Pop loop control targets
        //     self.break_target_stack.pop();
        //     self.continue_target_stack.pop();
        //
        //     // Set insertion point after the loop
        //     self.builder.position_at_end(after_bb);
        //
        //     Ok(())
        Ok(())
    }
}
