use std::collections::HashMap;

use crate::ast::{Statement, VariableDeclaration};
use anyhow::Result;

#[derive(Debug)]
pub struct Inner {
    pub variable_declaration: VariableDeclaration,
    pub immut_count: usize,
    pub mut_active: bool,
    pub moved: bool,
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

impl Inner {
    pub fn new(variable_declaration: VariableDeclaration) -> Self {
        Self {
            variable_declaration,
            immut_count: 0,
            mut_active: false,
            moved: false,
        }
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
                    moved: false,
                };

                self.variables.insert(var_decl.name.clone(), inner);
            }
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Inner> {
        self.variables.get(name)
    }

    pub fn borrow_immut(&mut self, name: &str) -> Result<(), anyhow::Error> {
        let var = self.variables.get_mut(name).ok_or_else(|| {
            anyhow::anyhow!(
                "Variable '{}' not found when attempting immutable borrow",
                name
            )
        })?;

        if var.mut_active {
            return Err(anyhow::anyhow!(
                "Cannot immutably borrow '{}'; it is currently mutably borrowed",
                name
            ));
        }

        var.immut_count += 1;
        Ok(())
    }

    pub fn borrow_mut(&mut self, name: &str) -> Result<(), anyhow::Error> {
        let var = self.variables.get_mut(name).ok_or_else(|| {
            anyhow::anyhow!(
                "Variable '{}' not found when attempting mutable borrow",
                name
            )
        })?;

        if var.mut_active || var.immut_count > 0 {
            return Err(anyhow::anyhow!(
                "Cannot mutably borrow '{}'; it has {} immutable borrows and mut_active is {}",
                name,
                var.immut_count,
                var.mut_active
            ));
        }

        var.mut_active = true;
        Ok(())
    }

    pub fn move_var(&mut self, name: &str) -> Result<()> {
        let var = self
            .variables
            .get_mut(name)
            .ok_or_else(|| anyhow::anyhow!("Variable '{}' not found", name))?;

        if var.moved {
            return Err(anyhow::anyhow!(
                "Variable '{}' was already moved and cannot be used again",
                name
            ));
        }

        if var.mut_active {
            return Err(anyhow::anyhow!(
                "Cannot move variable '{}' because it is mutably borrowed",
                name
            ));
        }

        var.moved = true;

        Ok(())
    }
}
