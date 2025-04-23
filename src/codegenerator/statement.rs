use super::CodeGenerator;
use crate::ast::Statement;
use anyhow::Result;

pub mod variable_declaration;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::VariableDeclaration(decl) => self.compile_variable_declaration(decl),
            Statement::Assignment(assign) => self.compile_variable_assignment(assign),
            Statement::Return(ret_stmt) => compile_ret_stmt(ret_stmt),
            Statement::FunctionCall(func_call) => compile_function_call(func_call),
            Statement::If(if_statement) => todo!(),
            Statement::While(while_statement) => todo!(),
            Statement::Break => todo!(),
            Statement::Continue => todo!(),
        }
    }
}