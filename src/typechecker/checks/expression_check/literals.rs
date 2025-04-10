use crate::ast::{Literal, Type};
use anyhow::Result;

/// Type check a literal expression
pub fn check_literal(literal: &Literal) -> Result<Type> {
    // Literals have straightforward type mapping
    Ok(Type::from(literal.clone()))
}


