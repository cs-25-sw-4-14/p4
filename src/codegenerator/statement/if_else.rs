use crate::{ast::IfStatement, codegenerator::CodeGenerator};
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_if_else(&mut self, if_stmt: &IfStatement) -> Result<()> {
        let parent_fn = self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_parent())
            .ok_or_else(|| anyhow::anyhow!("Not inside a function"))?;

        // Create basic blocks,
        // these are empty "blocks" in the LLVM IR that will contain code in each part of the if-else structure
        // 1 then, 1-2-3... else if, 1 else, 1 merge
        let then_bb = self.context.append_basic_block(parent_fn, "then");
        let mut else_if_bbs = vec![];
        for (i, _) in if_stmt.else_if_blocks.iter().enumerate() {
            else_if_bbs.push(
                self.context
                    .append_basic_block(parent_fn, &format!("else_if_{}", i)),
            );
        }
        let else_bb = if_stmt
            .else_block
            .as_ref()
            .map(|_| self.context.append_basic_block(parent_fn, "else"));

        let merge_bb = self.context.append_basic_block(parent_fn, "merge");

        // Evaluate the main condition
        // If cond == true → jump to then_bb
        // If cond == false → jump to else_if_bbs[0] or else_bb or merge_bb
        let cond = self.compile_expression(&if_stmt.condition)?; // Compile the condition expression
        let cond = cond.into_int_value();
        if !else_if_bbs.is_empty() {
            self.builder
                .build_conditional_branch(cond, then_bb, else_if_bbs[0]);
        } else if let Some(else_bb) = else_bb {
            self.builder
                .build_conditional_branch(cond, then_bb, else_bb);
        } else {
            self.builder
                .build_conditional_branch(cond, then_bb, merge_bb);
        }

        // Then block
        // Compiles the statements inside the "then" block
        self.builder.position_at_end(then_bb);
        for stmt in &if_stmt.then_block {
            self.compile_statement(stmt)?;
        }
        // If the last statement in the "then" block doesn't end with a return or break,
        // Adds an unconditional branch to the merge block
        if self
            .builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            self.builder.build_unconditional_branch(merge_bb);
        }

        //Else-if blocks
        // For each else-if block, condition is calculated
        // If cond == true → jump to else_if_body
        // If cond == false → jump to next else_if or else_bb or merge_bb
        for (i, else_if) in if_stmt.else_if_blocks.iter().enumerate() {
            self.builder.position_at_end(else_if_bbs[i]);

            let cond = self.compile_expression(&else_if.condition)?;
            let cond = cond.into_int_value();

            let next_bb = if i + 1 < else_if_bbs.len() {
                else_if_bbs[i + 1]
            } else if let Some(else_bb) = else_bb {
                else_bb
            } else {
                merge_bb
            };

            let body_bb = self
                .context
                .append_basic_block(parent_fn, &format!("else_if_body_{}", i));
            self.builder
                .build_conditional_branch(cond, body_bb, next_bb);

            // Else-if body
            self.builder.position_at_end(body_bb);
            for stmt in &else_if.body {
                self.compile_statement(stmt)?;
            }
            if self
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_none()
            {
                self.builder.build_unconditional_branch(merge_bb);
            }
        }

        // Else block (if it exists)
        // If the last statement in the "else" block doesn't end with a return or break,
        // Adds an unconditional branch to the merge block
        if let Some(else_bb) = else_bb {
            self.builder.position_at_end(else_bb);
            for stmt in if_stmt.else_block.as_ref().unwrap() {
                self.compile_statement(stmt)?;
            }
            if self
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_none()
            {
                self.builder.build_unconditional_branch(merge_bb);
            }
        }

        // 5. Move builder to merge block
        self.builder.position_at_end(merge_bb);

        Ok(())
    }
}
