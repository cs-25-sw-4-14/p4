use std::collections::HashMap;

use crate::ast::{EnumDef, Program};

pub struct ETable {
    pub enums: HashMap<String, EnumDef>,
}

impl Default for ETable {
    fn default() -> Self {
        Self::new()
    }
}

impl ETable {
    pub fn new() -> Self {
        Self {
            enums: HashMap::new(),
        }
    }

    pub fn walk(&mut self, program: Program) {
        for def in program.definitions {
            if let crate::ast::TopLevelDefinition::Enum(enum_def) = def {
                self.enums.insert(enum_def.name.clone(), enum_def.clone());
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&EnumDef> {
        self.enums.get(name)
    }
}
