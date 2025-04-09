use std::collections::HashMap;

use crate::ast::{FunctionDef, Program};

use super::vtable::VTable;

pub struct FunctionEntry {
    pub def: FunctionDef,
    pub vtable: VTable,
}

pub struct Ftable {
    pub functions: HashMap<String, FunctionEntry>,
}

impl Ftable {
    pub fn new() -> Self {
        Ftable {
            functions: HashMap::new(),
        }
    }

    pub fn walk(&mut self, program: Program) {
        for def in program.definitions {
            match def {
                crate::ast::TopLevelDefinition::Function(func) => {

                    let mut vtable = VTable::new();
                    vtable.walk(func.body.clone());

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
