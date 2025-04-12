use crate::{ast::{StructFieldAccess, Type, VariableDeclaration,ReturnStatement, Literal, Expression},
            typechecker::{function_table::FTable, struct_table::STable, variable_table::VTable}};
use anyhow::Result;

use crate::typechecker::expression_check;


pub fn check(return_statment: &ReturnStatement, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {
    
    // Check if the return type matches the function's return type
    
        
    if let Some(return_type) = expression_check::check(&return_statment.value, v_table, f_table, s_table).ok() {
        Ok(return_type)
    } else {
        return Err(anyhow::anyhow!("Return type not found"));
    }

    
}



#[cfg(test)]
pub fn check_return_statement() {
    let mut v_table = VTable::new();
    let f_table = FTable::new();
    let mut s_table = STable::new();

    let return_statement = ReturnStatement {
        value: Expression::Literal(Literal::Int(42)),
    };

    let result = check(&return_statement, &v_table, &f_table, &s_table);
    assert!(result.is_ok());
}