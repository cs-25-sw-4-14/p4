use std::collections::HashMap;
use crate::ast::{FunctionDef, Program, TopLevelDefinition};

pub struct FTable {
    pub functions: HashMap<String, FunctionDef>,
}

impl FTable {
    pub fn new() -> Self {
        FTable {
            functions: HashMap::new(),
        }
    }

    pub fn walk(&mut self, program: Program) {
        for def in program.definitions {
            match def {
                TopLevelDefinition::Function(func) => {

                    self.functions.insert(func.name.clone(), func.clone());

                }
                _ => {}
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&FunctionDef> {
        self.functions.get(name)
    }
}
