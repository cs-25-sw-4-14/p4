use std::collections::HashMap;

use crate::ast::VariableDeclaration;

#[derive(Debug)]
pub struct VTable {
    pub variables: HashMap<String, VariableDeclaration>,
}

impl Default for VTable {
    fn default() -> Self {
        Self::new()
    }
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
