use crate::ast::{Program, Expression, Statement, Type, TopLevelDefinition};
use anyhow::Result;

use super::tables::function_table::Ftable;
use super::tables::struct_table::STable;
use super::tables::variable_table::VTable;

pub struct Typechecker {
    pub ftable: Ftable,
    pub stable: STable,
}

impl Typechecker {
    pub fn new() -> Self {
        Typechecker {
            ftable: Ftable::new(),
            stable: STable::new(),
        }
    }

    pub fn walk(&mut self, program: Program) -> Result<()> {
        // First pass: populate symbol tables
        self.ftable.walk(program.clone());
        self.stable.walk(program.clone());

        // Debug output
        println!("Function Table: {:#?}", self.ftable.functions);
        println!("Struct Table: {:#?}", self.stable.structs);
        
        // Second pass: type check everything
        self.check_program(program)
    }
    
    // Helper method to check the whole program
    fn check_program(&self, program: Program) -> Result<()> {
        for def in program.definitions {
            match def {
                TopLevelDefinition::Function(func) => {
                    // Get this function's variable table
                    let vtable = match self.ftable.get(&func.name) {
                        Some(entry) => &entry.vtable,
                        None => continue, // Should never happen since we built the ftable
                    };
                    
                    // Check the function body
                    for stmt in &func.body {
                        self.check_statement(stmt, vtable)?;
                    }
                },
                TopLevelDefinition::Struct(_) => {
                    // Structs don't need additional checking after they're added to the stable
                }
            }
        }
        
        // Check if there's a main function
        if let Some(main_entry) = self.ftable.get("main") {
            if !main_entry.def.params.is_empty() {
                return Err(anyhow::anyhow!("main function should not have parameters"));
            }
            
            // Check if main's return type is Void
            match &main_entry.def.return_type {
                None => {}, // This is fine, None represents Void
                Some(t) => {
                    if let Type::Custom(name) = t {
                        if name != "Void" {
                            return Err(anyhow::anyhow!("main function should return Void, not {:?}", t));
                        }
                    } else {
                        return Err(anyhow::anyhow!("main function should return Void, not {:?}", t));
                    }
                }
            }
        } else {
            return Err(anyhow::anyhow!("Program must have a main function"));
        }
        
        Ok(())
    }
    
    // Helper method for checking expressions
    pub fn check_expression(&self, expr: &Expression, vtable: &VTable) -> Result<Type> {
        super::checks::expression_check::check(expr, vtable, &self.ftable, &self.stable)
    }
    
    // Helper method for checking statements
    pub fn check_statement(&self, stmt: &Statement, vtable: &VTable) -> Result<()> {
        super::checks::statement_check::check(stmt, vtable, &self.ftable, &self.stable)
    }
}
