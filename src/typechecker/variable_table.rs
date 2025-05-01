use std::collections::HashMap;

use crate::ast::{Statement, VariableDeclaration};

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

    pub fn walk(&mut self, statements: Vec<Statement>) -> Result<(), anyhow::Error> {
        for s in statements {
            if let Statement::VariableDeclaration(var_decl, _) = s {
                if self.variables.contains_key(&var_decl.name) {
                    return Err(anyhow::anyhow!(
                        "Variable {} already declared",
                        var_decl.name
                    ))?;
                }
                self.variables.insert(var_decl.name.clone(), var_decl);
            }
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&VariableDeclaration> {
        self.variables.get(name)
    }
}
