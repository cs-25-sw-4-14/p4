use std::collections::HashMap;

use crate::ast::{Statement, VariableDeclaration};

#[derive(Debug)]
pub struct Inner {
    pub variable_declaration: VariableDeclaration,
    pub immut_count: usize,
    pub mut_active: bool,
}

#[derive(Debug)]
pub struct VTable {
    pub variables: HashMap<String, Inner>,
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

                let inner = Inner {
                    variable_declaration: var_decl.clone(),
                    immut_count: 0,
                    mut_active: false,
                };

                self.variables.insert(var_decl.name.clone(), inner);
            }
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Inner> {
        self.variables.get(name)
    }
}
