use std::collections::HashMap;

use crate::ast::{Program, StructDef};

pub struct STable {
    pub structs: HashMap<String, StructDef>,
}

impl Default for STable {
    fn default() -> Self {
        Self::new()
    }
}

impl STable {
    pub fn new() -> Self {
        Self {
            structs: HashMap::new(),
        }
    }

    pub fn walk(&mut self, program: Program) {
        for def in program.definitions {
            if let crate::ast::TopLevelDefinition::Struct(struct_def) = def {
                self.structs
                    .insert(struct_def.name.clone(), struct_def.clone());
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&StructDef> {
        self.structs.get(name)
    }
}
