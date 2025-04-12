use crate::{ast::{StructFieldAccess, Type, VariableDeclaration}, typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable}};
use anyhow::Result;
use crate::ast::ReturnStatement;




pub fn check(return_statment: &ReturnStatement, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {
    
    



    Ok(Type::U16)

    
}