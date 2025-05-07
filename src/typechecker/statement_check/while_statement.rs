use crate::ast::{Type, WhileStatement};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_while_statement(&mut self, declaration: &WhileStatement) -> Result<()> {
        let condition_type = self.check_expression(&declaration.condition)?;
        if condition_type != Type::Bool {
            return Err(anyhow::anyhow!(
                "Condition type is not bool: {:?}",
                condition_type
            ));
        }
        // Check the body of the while statement
        for statement in &declaration.body {
            self.check_statement(statement)?;
        }

        Ok(())
    }
}
