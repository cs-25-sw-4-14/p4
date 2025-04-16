use std::collections::HashMap;
use crate::ast::{FunctionDef, Program, TopLevelDefinition};
use super::variable_table::VTable;

#[derive(Debug)]
pub struct FunctionEntry {
    pub def: FunctionDef,
    pub vtable: VTable,
}

pub struct FTable {
    pub functions: HashMap<String, FunctionEntry>,
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

                    let mut vtable = VTable::new();
                

                    self.functions.insert(func.name.clone(), FunctionEntry {
                        def: func.clone(),
                        vtable,
                    });

                }
                _ => {}
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&FunctionEntry> {
        self.functions.get(name)
    }
}
