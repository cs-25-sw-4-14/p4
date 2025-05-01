use crate::ast::{FunctionDef, Program, TopLevelDefinition};
use std::collections::HashMap;

pub struct FTable {
    pub functions: HashMap<String, FunctionDef>,
}

impl Default for FTable {
    fn default() -> Self {
        Self::new()
    }
}

impl FTable {
    pub fn new() -> Self {
        FTable {
            functions: HashMap::new(),
        }
    }

    pub fn walk(&mut self, program: Program) {
        for def in program.definitions {
            if let TopLevelDefinition::Function(func) = def {
                self.functions.insert(func.name.clone(), func.clone());
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(name)
    }
}
