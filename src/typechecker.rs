use crate::ast::Program;

pub mod function_table;
pub mod struct_table;
pub mod variable_table;
pub mod expression_check;
pub mod statement_check;

pub struct Typechecker {
    pub ftable: function_table::Ftable,
    pub stable: struct_table::STable,
}

impl Typechecker {
    pub fn new() -> Self {
        Typechecker {
            ftable: function_table::Ftable::new(),
            stable: struct_table::STable::new(),
        }
    }

    pub fn walk(&mut self, program: Program) {
        self.ftable.walk(program.clone());
        self.stable.walk(program.clone());
    }
}
