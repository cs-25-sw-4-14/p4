use std::collections::HashMap;

use crate::ast::{Statement, VariableDeclaration};
use anyhow::Result;

#[derive(Debug)]
pub struct VTable {
    pub variables: HashMap<String, VariableDeclaration>,
}

impl VTable {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    

    pub fn get(&self, name: &str) -> Option<&VariableDeclaration> {
        self.variables.get(name)
    }
}
