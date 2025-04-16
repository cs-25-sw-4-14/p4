use std::collections::HashMap;

use crate::ast::{Program, StructDef};

pub struct STable {
    pub structs: HashMap<String, StructDef>,
}

impl STable {
    pub fn new() -> Self {
        Self {
            structs: HashMap::new(),
        }
    }

    pub fn walk(&mut self, program: Program) {
        for def in program.definitions {
            match def {
                crate::ast::TopLevelDefinition::Struct(struct_def) => {
                    self.structs.insert(struct_def.name.clone(), struct_def.clone());
                }
                _ => {}
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&StructDef> {
        self.structs.get(name)
    }
}
