use crate::ast::Assignment;
use crate::ast::{Expression, Type};
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_assignment(&self, assignment: &Assignment, l: usize) -> Result<()> {
        let expression_value_type = self.check_expression(&assignment.value)?;
        let expression_target_type = self.check_expression(&assignment.target)?;

        if expression_value_type != expression_target_type {
            return Err(anyhow::anyhow!(
                "{}: Type mismatch: expected {:?}, found {:?}",
                self.debugger.display(l),
                expression_target_type,
                expression_value_type
            ));
        }

        if let Expression::Variable(ref var_name) = assignment.target {
            // Already handled
            if let Some(var_type) = self.v_table.get(var_name) {
                if var_type.var_type != expression_value_type {
                    return Err(anyhow::anyhow!(
                        "{}: Type mismatch: expected {:?}, found {:?}",
                        self.debugger.display(l),
                        var_type.var_type,
                        expression_value_type
                    ));
                }
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
        } else if let Expression::StructFieldAccess(ref access) = assignment.target {
            // Get type of struct expression
            let struct_type = self.check_expression(&access.struct_expr)?;
            if let Type::Custom(ref struct_name) = struct_type {
                if let Some(struct_def) = self.s_table.get(struct_name) {
                    if let Some(field) = struct_def
                        .fields
                        .iter()
                        .find(|f| f.name == access.field_name)
                    {
                        if field.field_type != expression_value_type {
                            return Err(anyhow::anyhow!(
                                "{}: Type mismatch: expected {:?}, found {:?}",
                                self.debugger.display(l),
                                field.field_type,
                                expression_value_type
                            ));
                        }
                    } else {
                        return Err(anyhow::anyhow!(
                            "{}: Field {} not found in struct {}",
                            self.debugger.display(l),
                            access.field_name,
                            struct_name
                        ));
                    }
                } else {
                    return Err(anyhow::anyhow!(
                        "{}: Struct {} not found",
                        self.debugger.display(l),
                        struct_name
                    ));
                }
            } else {
                return Err(anyhow::anyhow!(
                    "{}: Attempted to access field on non-struct type {:?}",
                    self.debugger.display(l),
                    struct_type
                ));
            }
        } else if let Expression::ArrayAccess(ref access) = assignment.target {
            // Look up the array variable
            if let Some(var_type) = self.v_table.get(&access.array_name) {
                if let Type::Array(boxed_element_type, _) = &var_type.var_type {
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
                        access.array_name
                    ));
                }
            } else {
                return Err(anyhow::anyhow!(
                    "{}: Array variable {} not declared",
                    self.debugger.display(l),
                    access.array_name
                ));
            }
        }

        Ok(())
    }
}
