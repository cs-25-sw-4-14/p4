use crate::ast::Throwaway;
use crate::typechecker::Typechecker;
use anyhow::Result;

impl<'a> Typechecker<'a> {
    pub fn check_throwaway(&mut self, throwaway: &Throwaway, l: usize) -> Result<()> {
        let expr = self.check_expression(&throwaway.expression);
        if expr.is_err() {
            return Err(anyhow::anyhow!(
                "{}: Error in thrown away expression: {:?}",
                self.debugger.display(l),
                expr
            ));
        }
        Ok(())
    }
}
