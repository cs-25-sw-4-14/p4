use crate::ast::{Read, Type};
use crate::typechecker::Typechecker;
use anyhow::{Result, anyhow};

impl Typechecker<'_> {
    pub fn check_read_file(&self, read_expr: &Read) -> Result<Type> {
        let filename_type = self.check_expression(&read_expr.value)?;

        if filename_type != Type::String {
            return Err(anyhow!(
                "Type Error: `read_file` expects a String expression for the filename, but found type '{:?}'",
                filename_type
            ));
        }

        Ok(Type::String)
    }
}
