use crate::ast::ArrayFieldAssignment;
use crate::ast::Type;
use crate::typechecker::Typechecker;
use anyhow::Result;
use anyhow::anyhow;

impl Typechecker<'_> {
    pub fn check_array_field_assignment(
        &mut self,
        assignment: &ArrayFieldAssignment,
        l: usize,
    ) -> Result<()> {
        let expression_value_type = self.check_expression(&assignment.value)?;

        let var = self.v_table.get(&assignment.target).ok_or(anyhow!(
            "{}: Variable {} not declared",
            self.debugger.display(l),
            assignment.target
        ))?;

        let expression_target_type = var.variable_declaration.var_type.clone();

        if let Type::Array(boxed_element_type, _) = &expression_target_type {
            if **boxed_element_type != expression_value_type {
                return Err(anyhow::anyhow!(
                    "{}: Type mismatch: expected {:?}, found {:?}",
                    self.debugger.display(l),
                    boxed_element_type,
                    expression_value_type
                ));
            }
        } else {
            return Err(anyhow::anyhow!(
                "{}: Variable {} is not an array",
                self.debugger.display(l),
                assignment.target
            ));
        }

        Ok(())
    }
}
