use crate::ast::EnumLiteral;
use crate::ast::Type;
use crate::typechecker::Typechecker;
use anyhow::Result;

impl<'a> Typechecker<'a> {
    pub fn check_enum_literal(&self, _enum_literal: &EnumLiteral) -> Result<Type> {
        todo!()
    }
}

#[cfg(test)]
pub mod vector_access_tests {}
