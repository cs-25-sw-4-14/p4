use crate::ast::VariableAssignment;
use crate::typechecker::Typechecker;
use anyhow::Result;
use anyhow::anyhow;

impl Typechecker<'_> {
    pub fn check_variable_assignment(
        &mut self,
        assignment: &VariableAssignment,
        l: usize,
    ) -> Result<()> {
        let expression_value_type = self.check_expression(&assignment.value)?;

        if assignment.target == "_" {
            return Ok(());
        }

        let var = self.v_table.get(&assignment.target).ok_or(anyhow!(
            "{}: Variable {} not declared",
            self.debugger.display(l),
            assignment.target
        ))?;

        let expression_target_type = var.variable_declaration.var_type.clone();

        if expression_value_type != expression_target_type {
            return Err(anyhow::anyhow!(
                "{}: Type mismatch: expected {:?}, found {:?}",
                self.debugger.display(l),
                expression_target_type,
                expression_value_type
            ));
        }
        Ok(())
    }
}
