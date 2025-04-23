use super::CodeGenerator;
use crate::ast::Statement;
use anyhow::Result;

pub mod assignment;
pub mod function_call;
pub mod return_statement;
pub mod variable_declaration;

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
            Statement::If(_if_statement) => todo!(),
            Statement::While(_while_statement) => todo!(),
            Statement::Break => todo!(),
            Statement::Continue => todo!(),
        };
        Ok(())
    }
}
