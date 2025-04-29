use super::CodeGenerator;
use crate::ast::Statement;
use anyhow::Result;

pub mod assignment;
pub mod break_statement;
pub mod continue_statement;
pub mod function_call;
pub mod if_else;
pub mod return_statement;
pub mod variable_declaration;
pub mod while_statement;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::VariableDeclaration(decl) => {
                self.compile_variable_declaration(decl)?;
            }
            Statement::Assignment(assign) => {
                self.compile_variable_assignment(assign)?;
            }
            Statement::Return(ret_stmt) => {
                self.compile_ret_stmt(ret_stmt)?;
            }
            Statement::FunctionCall(func_call) => {
                self.compile_function_call_stmt(func_call)?;
            }
            Statement::If(if_stmt) => self.compile_if_else(if_stmt)?,
            Statement::While(while_stmt) => self.compile_while_statement(while_stmt)?,
            Statement::Break => {
                // let break_target = *self.break_target_stack.last().unwrap();
                // self.builder.build_unconditional_branch(break_target);
            }
            Statement::Continue => {
                // let continue_target = *self.continue_target_stack.last().unwrap();
                // self.builder.build_unconditional_branch(continue_target);
            }
        };
        Ok(())
    }
}
