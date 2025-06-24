use crate::{ast::IfStatement, codegenerator::CodeGenerator};
use anyhow::Result;

impl CodeGenerator<'_> {
    pub fn compile_if_else(&mut self, if_stmt: &IfStatement) -> Result<()> {
        // Get the parent function from the current builder context.
        // Basic blocks need to be attached to a function.
        let parent_fn = self
            .builder
            .get_insert_block()               // Get current insertion block
            .and_then(|bb| bb.get_parent())   // Get the parent function of that block
            .ok_or_else(|| anyhow::anyhow!("Not inside a function"))?;

        // Create the basic block for the `then` branch of the if-statement.
        let then_bb = self.context.append_basic_block(parent_fn, "then");

        // Create basic blocks for each `else if` condition.
        let mut else_if_bbs = vec![];
        for (i, _) in if_stmt.else_if_blocks.iter().enumerate() {
            // Name each else-if block uniquely: else_if_0, else_if_1, etc.
            else_if_bbs.push(
                self.context
                    .append_basic_block(parent_fn, &format!("else_if_{}", i)),
            );
        }

        // Optionally create an `else` block if it exists.
        let else_bb = if_stmt
            .else_block
            .as_ref()
            .map(|_| self.context.append_basic_block(parent_fn, "else"));

        // Create a merge block that all branches will jump to at the end.
        let merge_bb = self.context.append_basic_block(parent_fn, "merge");

        // Compile the main `if` condition and cast it to an integer value for branching.
        let cond = self.compile_expression(&if_stmt.condition)?;
        let cond = cond.into_int_value();

        // Build the conditional branch from the main `if` condition:
        // if true → then_bb
        // if false → first else_if block or else block or merge block
        if !else_if_bbs.is_empty() {
            _ = self
                .builder
                .build_conditional_branch(cond, then_bb, else_if_bbs[0]);
        } else if let Some(else_bb) = else_bb {
            _ = self
                .builder
                .build_conditional_branch(cond, then_bb, else_bb);
        } else {
            _ = self
                .builder
                .build_conditional_branch(cond, then_bb, merge_bb);
        }

        // === Compile THEN branch ===
        self.builder.position_at_end(then_bb); // Move instruction cursor to `then_bb`
        for stmt in &if_stmt.then_block {
            self.compile_statement(stmt)?; // Compile each statement in then block
        }

        // If there's no terminator (e.g., return or branch), add an unconditional jump to merge.
        if self
            .builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            _ = self.builder.build_unconditional_branch(merge_bb);
        }

        // === Compile each ELSE IF ===
        for (i, else_if) in if_stmt.else_if_blocks.iter().enumerate() {
            // Move to the corresponding else_if condition block
            self.builder.position_at_end(else_if_bbs[i]);

            // Compile the else_if condition
            let cond = self.compile_expression(&else_if.condition)?;
            let cond = cond.into_int_value();

            // Decide where to jump if the condition is false:
            // → next else_if, or else block, or merge
            let next_bb = if i + 1 < else_if_bbs.len() {
                else_if_bbs[i + 1]
            } else if let Some(else_bb) = else_bb {
                else_bb
            } else {
                merge_bb
            };

            // Create a block for the body of this else_if
            let body_bb = self
                .context
                .append_basic_block(parent_fn, &format!("else_if_body_{}", i));

            // Conditional branch: if cond → body_bb, else → next_bb
            _ = self
                .builder
                .build_conditional_branch(cond, body_bb, next_bb);

            // Compile the else-if body
            self.builder.position_at_end(body_bb);
            for stmt in &else_if.body {
                self.compile_statement(stmt)?;
            }

            // Ensure body ends in jump to merge if not already terminated
            if self
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_none()
            {
                _ = self.builder.build_unconditional_branch(merge_bb);
            }
        }

        // === Compile ELSE block if it exists ===
        if let Some(else_bb) = else_bb {
            self.builder.position_at_end(else_bb); // Move to else block
            for stmt in if_stmt.else_block.as_ref().unwrap() {
                self.compile_statement(stmt)?; // Compile each statement
            }

            // Ensure else block ends with jump to merge
            if self
                .builder
                .get_insert_block()
                .unwrap()
                .get_terminator()
                .is_none()
            {
                _ = self.builder.build_unconditional_branch(merge_bb);
            }
        }

        // === Final merge block ===
        self.builder.position_at_end(merge_bb); // Continue code generation after if-else

        Ok(())
    }
}
