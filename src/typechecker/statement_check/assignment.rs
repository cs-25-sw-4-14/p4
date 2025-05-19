use crate::ast::Assignment;
use crate::ast::Expression;
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_assignment(&self, assignment: &Assignment, l: usize) -> Result<()> {
        let expression_value_type = self.check_expression(&assignment.value)?;
        let expression_target_type = self.check_expression(&assignment.target)?;

        // Check if the types match
        if expression_value_type != expression_target_type {
            return Err(anyhow::anyhow!(
                "{}: Type mismatch: expected {:?}, found {:?}",
                self.debugger.display(l),
                expression_target_type,
                expression_value_type
            ));
        }

        // Check if the target is a variable and if it is mutable
        if let Expression::Variable(ref var_name) = assignment.target {
            if let Some(var_type) = self.v_table.get(var_name) {
                if var_type.mutable.is_none() {
                    return Err(anyhow::anyhow!(
                        "{}: Variable {} is immutable",
                        self.debugger.display(l),
                        var_name
                    ));
                }
            } else {
                return Err(anyhow::anyhow!("Variable {} not declared", var_name));
            }
        }

        Ok(())
    }
}
