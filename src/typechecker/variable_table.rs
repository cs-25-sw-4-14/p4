use std::collections::HashMap;

use crate::ast::{Statement, VariableDeclaration};

pub struct VTable {
    pub variables: HashMap<String, VariableDeclaration>,
}

impl VTable {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn walk(&mut self, statements: Vec<Statement>) {
        for s in statements {
            match s {
                Statement::VariableDeclaration(var_decl) => {
                    self.variables.insert(var_decl.name.clone(), var_decl);
                }
                _ => {}
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&VariableDeclaration> {
        self.variables.get(name)
    }
}
