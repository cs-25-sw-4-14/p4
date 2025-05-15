use crate::ast::VariableDeclaration;
use crate::typechecker::Typechecker;
use anyhow::Result;
use crate::ast::Type;

impl Typechecker<'_> {
    pub fn check_variable_declaration(
        &mut self,
        declaration: &VariableDeclaration,
        l: usize
    ) -> Result<()> {
        // Check if the type of the value matches the declared type

        let value_type = self.check_expression(&declaration.value)?;

        match &declaration.var_type {
            Type::Vector(_, _) => {
                if let Type::Vector(_, len) = value_type {
                    let mut new_declaration = declaration.clone();
                    new_declaration.var_type = Type::Vector(
                        Box::new(new_declaration.var_type),
                        len
                    );
                    self.semantic_check_variable_already_declared_variable_declaration(
                        declaration
                    )?;

                    self.v_table.variables.insert(declaration.name.clone(), new_declaration);

                    Ok(())
                } else {
                    return Err(
                        anyhow::anyhow!(
                            "{}: Type mismatch: expected vector, found {:?}",
                            self.debugger.display(l),
                            value_type
                        )
                    );
                }
            }
            _ => {
                if value_type != declaration.var_type {
                    return Err(
                        anyhow::anyhow!(
                            "{}: Type mismatch: expected {:?}, found {:?}",
                            self.debugger.display(l),
                            declaration.var_type,
                            value_type
                        )
                    );
                }
                self.semantic_check_variable_already_declared_variable_declaration(declaration)?;

                self.v_table.variables.insert(declaration.name.clone(), declaration.clone());

                Ok(())
            }
        }
    }

    pub fn semantic_check_variable_already_declared_variable_declaration(
        &self,
        declaration: &VariableDeclaration
    ) -> Result<()> {
        // Check if the variable is already declared in the current scope
        if self.v_table.variables.contains_key(&declaration.name) {
            return Err(anyhow::anyhow!("Variable {} is already declared", declaration.name));
        }
        Ok(())
    }
}
