use crate::ast::Program;
use crate::ast::Statement;
use crate::ast::span_to_location;

pub mod expression_check;
pub mod function_table;
pub mod statement_check;
pub mod struct_table;
pub mod variable_table;

pub struct Typechecker {
    pub ftable: function_table::FTable,
    pub stable: struct_table::STable,
}

impl Typechecker {
    pub fn new() -> Self {
        Typechecker {
            ftable: function_table::FTable::new(),
            stable: struct_table::STable::new(),
        }
    }

    pub fn walk(&mut self, program: Program) {
        self.ftable.walk(program.clone());
        self.stable.walk(program.clone());

        println!("Function Table: {:#?}", self.ftable.functions);
        println!("Struct Table: {:#?}", self.stable.structs);
    }

    pub fn check_breaks_continues(stmt: &Statement, in_loop: bool, source_file: &str) -> Result<(), String> {
        match stmt {
            Statement::Break(l) => {
                if in_loop {
                    Ok(())
                } else {
                    let start = span_to_location(source_file, l).start;
                    Err(format!("`break` used outside of loop on line {} char {}", start.line, start.char).into())
                }
            }
            Statement::Continue(l) => {
                if in_loop {
                    Ok(())
                } else {
                    let start = span_to_location(source_file, l).start;
                    Err(format!("`continue` used outside of loop on line {} char {}", start.line, start.char).into())
                }
            }
            Statement::While(while_stmt) => {
                for stmt in &while_stmt.body {
                    Self::check_breaks_continues(stmt, true, source_file)?
                }
                Ok(())
            }
            Statement::If(if_stmt) => {
                for stmt in &if_stmt.then_block {
                    Self::check_breaks_continues(stmt, in_loop, source_file)?
                }
                for else_if in &if_stmt.else_if_blocks {
                    for stmt in &else_if.body {
                Self::check_breaks_continues(stmt, in_loop, source_file)?
                    }
                }
                if let Some(else_block) = &if_stmt.else_block {
                    for stmt in else_block {
                Self::check_breaks_continues(stmt, in_loop, source_file)?
                    }
                }
                Ok(())
            }
            _ => Ok(())
        }
    }
}
