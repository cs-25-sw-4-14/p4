use crate::ast::StructFieldAssignment;
use crate::ast::Type;
use crate::typechecker::Typechecker;
use anyhow::Result;
use anyhow::anyhow;

impl Typechecker<'_> {
    pub fn check_struct_field_assignment(
        &mut self,
        assignment: &StructFieldAssignment,
        l: usize,
    ) -> Result<()> {
        let expression_value_type = self.check_expression(&assignment.value)?;

        let var = self.v_table.get(&assignment.target).ok_or(anyhow!(
            "{}: Variable {} not declared",
            self.debugger.display(l),
            assignment.target
        ))?;

        let expression_target_type = var.variable_declaration.var_type.clone();

        if let Type::Custom(ref struct_name) = expression_target_type {
            if let Some(struct_def) = self.s_table.get(struct_name) {
                if let Some(field) = struct_def
                    .fields
                    .iter()
                    .find(|f| f.name == assignment.field_name)
                {
                    if field.field_type != expression_value_type {
                        return Err(anyhow::anyhow!(
                            "{}: Type mismatch: expected {:?}, found {:?}",
                            self.debugger.display(l),
                            field.field_type,
                            expression_value_type
                        ));
                    }
                    Ok(())
                } else {
                    return Err(anyhow::anyhow!(
                        "{}: Field {} not found in struct {}",
                        self.debugger.display(l),
                        assignment.field_name,
                        assignment.target
                    ));
                }
            } else {
                return Err(anyhow::anyhow!(
                    "{}: Struct {} not found",
                    self.debugger.display(l),
                    assignment.target
                ));
            }
        } else {
            return Err(anyhow::anyhow!(
                "{}: Variable {} is not a struct",
                self.debugger.display(l),
                assignment.target
            ));
        }
    }
}
