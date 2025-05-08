use crate::ast::Program;
use crate::debugger::Debugger;
use anyhow::Result;

pub mod borrow_checker;
pub mod enum_table;
pub mod expression_check;
pub mod function_check;
pub mod function_table;
pub mod statement_check;
pub mod struct_table;
pub mod variable_table;

pub struct Typechecker<'a> {
    pub f_table: function_table::FTable,
    pub s_table: struct_table::STable,
    pub e_table: enum_table::ETable,
    pub v_table: variable_table::VTable,
    pub borrow_checker: borrow_checker::BorrowChecker,
    pub debugger: &'a Debugger,
}

impl<'a> Typechecker<'a> {
    pub fn new(debugger: &'a Debugger) -> Self {
        Typechecker {
            f_table: function_table::FTable::new(),
            s_table: struct_table::STable::new(),
            e_table: enum_table::ETable::new(),
            v_table: variable_table::VTable::new(),
            borrow_checker: borrow_checker::BorrowChecker::new(),
            debugger,
        }
    }

    pub fn walk(&mut self, program: Program) -> Result<()> {
        self.f_table.walk(program.clone());
        self.s_table.walk(program.clone());

        for def in program.definitions.iter() {
            if let crate::ast::TopLevelDefinition::Function(func_def) = def {
                self.check_function(func_def)?;
            }
        }

        // borrow checker!!!...
        self.borrow_checker.check_program(&program)?;

        // println!("Function Table: {:#?}", self.ftable.functions);
        // println!("Struct Table: {:#?}", self.stable.structs);

        Ok(())
    }
}
