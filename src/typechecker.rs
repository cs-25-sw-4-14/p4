use crate::ast::Program;

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
}
