use crate::ast::{BinaryOperation, Type, BinaryOperator};
use crate::typechecker::tables::{function_table::Ftable, variable_table::VTable, struct_table::STable};
use super::super::expression_check;
use super::type_utils::{is_numeric_type, wider_numeric_type, types_match};
use anyhow::Result;

/// Type check a binary operation expression
pub fn check_binary_op(
    binary_operation: &BinaryOperation,
    vtable: &VTable,
    ftable: &Ftable,
    stable: &STable
) -> Result<Type> {
    let left_type = expression_check::check(&binary_operation.left, vtable, ftable, stable)?;
    let right_type = expression_check::check(&binary_operation.right, vtable, ftable, stable)?;
    
    match binary_operation.operator {
        // Arithmetic operations: +, -, *, /, %
        BinaryOperator::Add | BinaryOperator::Subtract | 
        BinaryOperator::Multiply | BinaryOperator::Divide | 
        BinaryOperator::Modulus => {
            // String concatenation with +
            if binary_operation.operator == BinaryOperator::Add && 
               left_type == Type::String && right_type == Type::String {
                return Ok(Type::String);
            }
            
            // Numeric operations
            if is_numeric_type(&left_type) && is_numeric_type(&right_type) {
                // For now, return the "wider" of the two types
                return Ok(wider_numeric_type(left_type, right_type));
            }
            
            Err(anyhow::anyhow!(
                "Invalid operand types for binary operation {:?}: {:?} and {:?}",
                binary_operation.operator, left_type, right_type
            ))
        },
        
        // Comparison operations: <, >, <=, >=
        BinaryOperator::LessThan | BinaryOperator::GreaterThan |
        BinaryOperator::LessThanOrEqual | BinaryOperator::GreaterThanOrEqual => {
            if (is_numeric_type(&left_type) && is_numeric_type(&right_type)) ||
               (left_type == Type::String && right_type == Type::String) {
                return Ok(Type::Bool);
            }
            
            Err(anyhow::anyhow!(
                "Invalid operand types for comparison operation {:?}: {:?} and {:?}",
                binary_operation.operator, left_type, right_type
            ))
        },
        
        // Equality operations: ==, !=
        BinaryOperator::Equal | BinaryOperator::NotEqual => {
            if types_match(&left_type, &right_type) {
                return Ok(Type::Bool);
            }
            
            Err(anyhow::anyhow!(
                "Cannot compare different types with {:?}: {:?} and {:?}",
                binary_operation.operator, left_type, right_type
            ))
        },
        
        // Logical operations: &&, ||
        BinaryOperator::And | BinaryOperator::Or => {
            if left_type == Type::Bool && right_type == Type::Bool {
                return Ok(Type::Bool);
            }
            
            Err(anyhow::anyhow!(
                "Logical operations require boolean operands, got {:?} and {:?}",
                left_type, right_type
            ))
        },
    }
}

