use crate::ast::Program;
use crate::ast::Statement;
use anyhow::Result;

pub mod expression_check;
pub mod function_check;
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

    pub fn walk(&mut self, program: Program) -> Result<()> {
        self.ftable.walk(program.clone());
        self.stable.walk(program.clone());

        for def in program.definitions {
            match def {
                crate::ast::TopLevelDefinition::Function(func_def) => {
                    function_check::check(&func_def, &mut self.ftable, &self.stable)?;
                }
                _ => {}
            }
        }

        println!("Function Table: {:#?}", self.ftable.functions);
        println!("Struct Table: {:#?}", self.stable.structs);

        Ok(())
    }

    pub fn check_breaks_continues(stmt: &Statement, in_loop: bool) -> Result<(), String> {
        match stmt {
            Statement::Break => {
                if in_loop {
                    Ok(())
                } else {
                    Err("`break` used outside of loop".into())
                }
            }
            Statement::Continue => {
                if in_loop {
                    Ok(())
                } else {
                    Err("`continue` used outside of loop".into())
                }
            }
            Statement::While(while_stmt) => {
                for stmt in &while_stmt.body {
                    Self::check_breaks_continues(stmt, true)?
                }
                Ok(())
            }
            Statement::If(if_stmt) => {
                for stmt in &if_stmt.then_block {
                    Self::check_breaks_continues(stmt, in_loop)?
                }
                for else_if in &if_stmt.else_if_blocks {
                    for stmt in &else_if.body {
                        Self::check_breaks_continues(stmt, in_loop)?
                    }
                }
                if let Some(else_block) = &if_stmt.else_block {
                    for stmt in else_block {
                        Self::check_breaks_continues(stmt, in_loop)?
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
