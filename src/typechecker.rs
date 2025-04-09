pub mod ftable;
pub mod stable;
pub mod vtable;
pub mod echeck;

pub struct Typechecker {
    pub ftable: ftable::Ftable,
    pub stable: stable::STable,
}

impl Typechecker {
    pub fn new() -> Self {
        Typechecker {
            ftable: ftable::Ftable::new(),
            stable: stable::STable::new(),
        }
    }

    pub fn walk(&mut self, program: crate::ast::Program) {
        self.ftable.walk(program.clone());
        self.stable.walk(program.clone());
    }
}
