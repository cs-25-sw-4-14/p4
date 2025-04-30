use crate::{ast::IfStatement, codegenerator::CodeGenerator};
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_if_else(&mut self, if_stmt: &IfStatement) -> Result<()> {
        let parent_fn = self
            .builder
            .get_insert_block()
            .and_then(|bb| bb.get_parent())
            .ok_or_else(|| anyhow::anyhow!("Not inside a function"))?;

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

        let cond = self.compile_expression(&if_stmt.condition)?;
        let cond = cond.into_int_value();
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

        self.builder.position_at_end(then_bb);
        for stmt in &if_stmt.then_block {
            self.compile_statement(stmt)?;
        }
        if self
            .builder
            .get_insert_block()
            .unwrap()
            .get_terminator()
            .is_none()
        {
            _ = self.builder.build_unconditional_branch(merge_bb);
        }

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
            _ = self
                .builder
                .build_conditional_branch(cond, body_bb, next_bb);

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
                _ = self.builder.build_unconditional_branch(merge_bb);
            }
        }

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
                _ = self.builder.build_unconditional_branch(merge_bb);
            }
        }

        self.builder.position_at_end(merge_bb);

        Ok(())
    }
}
