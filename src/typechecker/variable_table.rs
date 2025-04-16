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

    pub fn walk(&mut self, statements: Vec<Statement>) -> Result<()> {
        for s in statements {
            match s {
                Statement::VariableDeclaration(var_decl) => {
                    if (self.variables.contains_key(&var_decl.name)) {
                        return Err(anyhow::anyhow!("Variable {} already declared", var_decl.name))?; 
                    }
                    self.variables.insert(var_decl.name.clone(), var_decl);

                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&VariableDeclaration> {
        self.variables.get(name)
    }
}
