use crate::ast::Program;
use crate::debugger::Debugger;
use anyhow::Result;

pub mod borrow_checker;
pub mod expression_check;
pub mod function_check;
pub mod function_table;
pub mod statement_check;
pub mod struct_table;
pub mod variable_table;

pub struct Typechecker {
    pub ftable: function_table::FTable,
    pub stable: struct_table::STable,
    pub borrow_checker: borrow_checker::BorrowChecker,
}

impl Typechecker {
    pub fn new() -> Self {
        Typechecker {
            ftable: function_table::FTable::new(),
            stable: struct_table::STable::new(),
            borrow_checker: borrow_checker::BorrowChecker::new(),
        }
    }

    pub fn walk(&mut self, program: Program, debugger: Debugger) -> Result<()> {
        self.ftable.walk(program.clone());
        self.stable.walk(program.clone());

        for def in program.definitions.iter() {
            match def {
                crate::ast::TopLevelDefinition::Function(func_def) => {
                    function_check::check(&func_def, &mut self.ftable, &self.stable, &debugger)?;
                }
                _ => {}
            }
        }

        // borrow checker!!!...
        self.borrow_checker.check_program(&program)?;

        // println!("Function Table: {:#?}", self.ftable.functions);
        // println!("Struct Table: {:#?}", self.stable.structs);

        Ok(())
    }
}
