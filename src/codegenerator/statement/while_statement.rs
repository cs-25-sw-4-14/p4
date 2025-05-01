use crate::{ast::WhileStatement, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::IntPredicate;

impl CodeGenerator<'_> {
    pub fn compile_while_statement(&mut self, while_stmt: &WhileStatement) -> Result<()> {
        let current_fn = match self.current_function {
            Some(function) => function,
            None => return Err(anyhow!("No active function context for while statement")),
        };

        let cond_bb = self.context.append_basic_block(current_fn, "loop_cond");
        let body_bb = self.context.append_basic_block(current_fn, "loop_body");
        let after_bb = self.context.append_basic_block(current_fn, "after_loop");

        self.break_target_stack.push(after_bb);
        self.continue_target_stack.push(cond_bb);

        _ = self.builder.build_unconditional_branch(cond_bb);

        self.builder.position_at_end(cond_bb);
        let condition_value = self.compile_expression(&while_stmt.condition)?;

        let condition = match condition_value {
            inkwell::values::BasicValueEnum::IntValue(int_val) => {
                let zero = int_val.get_type().const_zero();
                self.builder
                    .build_int_compare(IntPredicate::NE, int_val, zero, "loop_cond")
            }
            inkwell::values::BasicValueEnum::FloatValue(float_val) => {
                let zero = float_val.get_type().const_zero();
                self.builder.build_float_compare(
                    inkwell::FloatPredicate::ONE,
                    float_val,
                    zero,
                    "loop_cond",
                )
            }
            _ => return Err(anyhow!("Unsupported condition type in while statement")),
        };

        _ = self
            .builder
            .build_conditional_branch(condition.unwrap(), body_bb, after_bb);

        self.builder.position_at_end(body_bb);
        for stmt in &while_stmt.body {
            self.compile_statement(stmt)?;
        }

        let current_block = self.builder.get_insert_block().unwrap();
        if current_block.get_terminator().is_none() {
            _ = self.builder.build_unconditional_branch(cond_bb);
        }

        self.break_target_stack.pop();
        self.continue_target_stack.pop();

        self.builder.position_at_end(after_bb);

        Ok(())
    }
}
