use super::{Typechecker, variable_table::Inner};
use crate::ast::{FunctionDef, ReturnStatement, Statement, Type, VariableDeclaration};
use anyhow::{Result, anyhow};
use std::collections::HashSet;

impl Typechecker<'_> {
    pub fn check_function(&mut self, function: &FunctionDef) -> Result<()> {
        // check if function exists in the function table
        if self.f_table.get(&function.name).is_none() {
            return Err(anyhow!(
                "Function {} not defined in function table",
                function.name
            ));
        }

        //Clear variables
        self.v_table.variables.clear();

        //check for duplicate parameter names
        check_duplicate_params(function)?;

        // add parameters to the variable table
        for param in &function.params {
            self.v_table.variables.insert(
                param.name.clone(),
                Inner::new(VariableDeclaration {
                    trivial: None,
                    mutable: None,
                    var_type: param.param_type.clone(),
                    name: param.name.clone(),
                    value: crate::ast::Expression::Literal(crate::ast::Literal::Int(0)), // bare noget 🤷‍♂️ ligegyldigt!
                }),
            );
        }

        // type-check the function body
        self.check_function_body(function)?;

        // check return statements
        self.check_return_statements(function)?;

        for stmt in &function.body {
            self.check_breaks_continues(stmt, false)?;
        }

        Ok(())
    }

    fn check_function_body(&mut self, function: &FunctionDef) -> Result<()> {
        for statement in &function.body {
            // statement_check::check(statement, v_table, f_table, s_table, debugger)?;
            self.check_statement(statement)?;
        }

        Ok(())
    }

    fn check_return_statements(&mut self, function: &FunctionDef) -> Result<()> {
        let return_type = &function.return_type;
        let mut has_return = false;

        for statement in &function.body {
            if let Statement::Return(ret, _) = statement {
                has_return = true;
                self.check_return_type(ret, return_type)?;
            }
        }

        //check if function with non-void return type has at least one return statement
        if return_type.is_some() && !has_return {
            return Err(anyhow!(
                "Function {} has return type {:?} but no return statement",
                function.name,
                return_type
            ));
        }

        Ok(())
    }

    fn check_return_type(
        &mut self,
        ret: &ReturnStatement,
        expected_type: &Option<Type>,
    ) -> Result<()> {
        match (expected_type, &ret.value) {
            // Case 1: Function returns void, return statement should be empty
            (None, Some(_)) => Err(anyhow!("Function has void return type but returns a value")),

            // Case 2: Function returns non-void, return statement should have matching type
            (Some(expected), Some(expr)) => {
                let actual_type = self.check_expression(expr)?;
                if *expected != actual_type {
                    Err(anyhow!(
                        "Return type mismatch: expected {:?}, found {:?}",
                        expected,
                        actual_type
                    ))
                } else {
                    Ok(())
                }
            }

            // Case 3: Function returns non-void, but return statement is empty
            (Some(expected), None) => Err(anyhow!(
                "Function has return type {:?} but return statement is empty",
                expected
            )),

            // Case 4: Function returns void, and return statement is empty (valid)
            (None, None) => Ok(()),
        }
    }

    pub fn check_breaks_continues(&self, stmt: &Statement, in_loop: bool) -> Result<()> {
        match stmt {
            Statement::Break(l) => {
                if in_loop {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "`break` used outside of loop on {}",
                        self.debugger.display(*l),
                    ))
                }
            }
            Statement::Continue(l) => {
                if in_loop {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "`continue` used outside of loop on {}",
                        self.debugger.display(*l)
                    ))
                }
            }
            Statement::While(while_stmt, _) => {
                for stmt in &while_stmt.body {
                    self.check_breaks_continues(stmt, true)?
                }
                Ok(())
            }
            Statement::If(if_stmt, _) => {
                for stmt in &if_stmt.then_block {
                    self.check_breaks_continues(stmt, in_loop)?
                }
                for else_if in &if_stmt.else_if_blocks {
                    for stmt in &else_if.body {
                        self.check_breaks_continues(stmt, in_loop)?
                    }
                }
                if let Some(else_block) = &if_stmt.else_block {
                    for stmt in else_block {
                        self.check_breaks_continues(stmt, in_loop)?
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

fn check_duplicate_params(function: &FunctionDef) -> Result<()> {
    let mut param_names = HashSet::new();

    for param in &function.params {
        if !param_names.insert(&param.name) {
            return Err(anyhow!(
                "Duplicate parameter name: {} in function {}",
                param.name,
                function.name
            ));
        }
    }

    Ok(())
}
