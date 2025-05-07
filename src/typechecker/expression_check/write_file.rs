use crate::ast::{Type, Write};
use crate::typechecker::Typechecker;
use anyhow::{Result, anyhow};

impl Typechecker<'_> {
    pub fn check_write_file(&self, write_expr: &Write) -> Result<Type> {
        // Check the type of the 'value' expression (the content to write)
        let value_type = self.check_expression(&write_expr.value)?;

        // Validate that the value expression evaluates to a String
        if value_type != Type::String {
            return Err(anyhow!(
                "Type Error: `write_file` expects a String expression for the 'value' argument (content), but found type '{:?}'",
                value_type
            ));
        }

        // Check the type of the 'target' expression (the filename)
        let target_type = self.check_expression(&write_expr.target)?;

        // Validate that the target expression evaluates to a String
        if target_type != Type::String {
            return Err(anyhow!(
                "Type Error: `write_file` expects a String expression for the 'target' argument (filename), but found type '{:?}'",
                target_type
            ));
        }

        Ok(Type::I32)
    }
}
