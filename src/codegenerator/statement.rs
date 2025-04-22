use super::CodeGenerator;
use crate::ast::Statement;
use anyhow::Result;

pub mod variable_declaration;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::VariableDeclaration(decl) => self.compile_variable_declaration(decl),
            Statement::Assignment(assign) => todo!(),
            Statement::Return(ret_stmt) => todo!(),
            Statement::FunctionCall(func_call) => todo!(),
            Statement::If(if_statement) => todo!(),
            Statement::While(while_statement) => todo!(),
            Statement::Break => todo!(),
            Statement::Continue => todo!(),
        }
    }
}
