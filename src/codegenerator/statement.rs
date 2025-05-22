use super::CodeGenerator;
use crate::ast::Statement;
use anyhow::Result;

pub mod array_field_assignment;
pub mod break_statement;
pub mod continue_statement;
pub mod function_call;
pub mod if_else;
pub mod print;
pub mod return_statement;
pub mod struct_field_assignment;
pub mod variable_assignment;
pub mod variable_declaration;
pub mod vector_insert;
pub mod vector_remove;
pub mod while_statement;

impl CodeGenerator<'_> {
    pub fn compile_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::VariableAssignment(assign, _) => {
                self.compile_variable_assignment(assign)?;
            }
            Statement::StructFieldAssignment(assign, _) => {
                self.compile_struct_field_assignment(assign)?;
            }
            Statement::ArrayFieldAssignment(assign, _) => {
                self.compile_array_field_assignment(assign)?;
            }
            Statement::Throwaway(throwaway, _) => {
                self.compile_expression(&throwaway.expression)?;
            }
            Statement::Debug(expr, _) => {
                self.compile_print_stmt(expr)?;
            }
            Statement::VariableDeclaration(decl, _) => {
                self.compile_variable_declaration(decl)?;
            }
            Statement::Return(ret_stmt, _) => {
                self.compile_ret_stmt(ret_stmt)?;
            }
            Statement::FunctionCall(func_call, _) => {
                self.compile_function_call_stmt(func_call)?;
            }
            Statement::If(if_stmt, _) => self.compile_if_else(if_stmt)?,
            Statement::While(while_stmt, _) => self.compile_while_statement(while_stmt)?,
            Statement::Break(_) => {
                let break_target = *self.break_target_stack.last().unwrap();
                _ = self.builder.build_unconditional_branch(break_target);
            }
            Statement::Continue(_) => {
                let continue_target = *self.continue_target_stack.last().unwrap();
                _ = self.builder.build_unconditional_branch(continue_target);
            }
            Statement::VectorInsert(vector_insert, _) => {
                self.compile_vector_insert(vector_insert)?
            }
            Statement::VectorRemove(vector_remove, _) => {
                self.compile_vector_remove(vector_remove)?
            }
            // TODO: Handle Match
            Statement::Match(_, _) => todo!(),
        };
        Ok(())
    }
}
