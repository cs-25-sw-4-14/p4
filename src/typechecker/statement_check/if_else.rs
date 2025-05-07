use crate::ast::{IfStatement, Type};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl<'a> Typechecker<'a> {
    pub fn check_if_else(&mut self, declaration: &IfStatement) -> Result<()> {
        let condition_type = self.check_expression(&declaration.condition)?;
        if condition_type != Type::Bool {
            return Err(anyhow::anyhow!(
                "Condition type is not bool: {:?}",
                condition_type
            ));
        }
        // Check the body of the if statement
        for statement in &declaration.then_block {
            self.check_statement(statement)?;
        }
        // Check the else if blocks
        for else_if in &declaration.else_if_blocks {
            let condition_type = self.check_expression(&else_if.condition)?;
            if condition_type != Type::Bool {
                return Err(anyhow::anyhow!(
                    "Condition type is not bool: {:?}",
                    condition_type
                ));
            }
            for statement in &else_if.body {
                self.check_statement(statement)?;
            }
        }
        // Check the else block if it exists
        if let Some(else_block) = &declaration.else_block {
            for statement in else_block {
                self.check_statement(statement)?;
            }
        }

        Ok(())
    }
}
