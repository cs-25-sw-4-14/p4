use super::Typechecker;
use crate::ast::{StructDef, Type};
use anyhow::{Result, anyhow, bail};

impl Typechecker<'_> {
    pub fn check_struct(&mut self, struct_def: &StructDef) -> Result<()> {
        // check if struct exists in the struct table
        if self.s_table.get(&struct_def.name).is_none() {
            return Err(anyhow!(
                "Function {} not defined in function table",
                struct_def.name
            ));
        }

        for field in &struct_def.fields {
            match field.field_type {
                Type::Reference(_, _) => bail!("Struct fields cannot be references"),
                _ => {}
            }
        }
        Ok(())
    }
}
