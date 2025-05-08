use super::Typechecker;
use crate::ast::Statement;
use anyhow::Result;

mod assignment;
mod function_call;
mod if_else;
mod throwaway;
mod variable_declaration;
mod vector_insert;
mod vector_remove;
mod while_statement;

impl Typechecker<'_> {
    pub fn check_statement(&mut self, expression: &Statement) -> Result<()> {
        match expression {
            Statement::Throwaway(expression, l) => self.check_throwaway(expression, *l)?,
            Statement::VariableDeclaration(variable_declaration, l) => {
                self.check_variable_declaration(variable_declaration, *l)?
            }
            Statement::Assignment(variable_assignment, l) => {
                self.check_assignment(variable_assignment, *l)?
            }
            Statement::VectorInsert(_vector_insert, l) => {
                self.check_vector_insert(_vector_insert, *l)?
            }
            Statement::VectorRemove(vector_delete, l) => {
                self.check_vector_remove(vector_delete, *l)?
            }

            Statement::If(if_else, _) => self.check_if_else(if_else)?,
            Statement::While(while_statement, _) => self.check_while_statement(while_statement)?,
            Statement::Break(_) => {}
            Statement::Continue(_) => {}
            Statement::Return(_, _) => {}
            Statement::FunctionCall(function_call, _) => {
                self.check_function_call_stmt(function_call)?;
            }
            Statement::Debug(print, _) => {
                self.check_expression(&print.expression)?;
            }
            // TODO: Handle match
            Statement::Match(_, _) => todo!(),
        }

        Ok(())
    }
}
