use crate::ast::{UnaryOperation, Type, UnaryOperator};
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use super::super::expression_check;
use super::type_utils::is_numeric_type;
use anyhow::Result;

/// Type check a unary operation expression
pub fn check_unary_op(
    unary_operation: &UnaryOperation,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<Type> {
    let operand_type = expression_check::check(&unary_operation.operand, vtable, ftable, stable)?;
    
    match unary_operation.operator {
        UnaryOperator::Negate => {
            if is_numeric_type(&operand_type) {
                return Ok(operand_type);
            }
            
            Err(anyhow::anyhow!(
                "Cannot negate non-numeric type {:?}", operand_type
            ))
        },
        
        UnaryOperator::Not => {
            if operand_type == Type::Bool {
                return Ok(Type::Bool);
            }
            
            Err(anyhow::anyhow!(
                "Logical NOT requires boolean operand, got {:?}", operand_type
            ))
        },
    }
}

