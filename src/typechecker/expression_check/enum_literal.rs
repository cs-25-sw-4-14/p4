use crate::ast::EnumLiteral;
use crate::ast::Type;
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_enum_literal(&self, _enum_literal: &EnumLiteral) -> Result<Type> {
        todo!()
    }
}

#[cfg(test)]
pub mod vector_access_tests {}
