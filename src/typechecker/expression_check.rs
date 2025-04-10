use crate::ast::{Expression, Type};
use super::{function_table::Ftable, variable_table::VTable};
use anyhow::Result;



pub fn check(expression: &Expression, vtable: &VTable, ftable: &Ftable) -> Result<Type> {

    match expression {
        Expression::Literal(literal) => Ok(Type::from(literal.clone())),
        Expression::Variable(name) => {
            if let Some(var) = vtable.get(name) {
                Ok(var.var_type.clone())
            } else {
                Err(anyhow::anyhow!("Variable {} not found in vtable", name))
            }
        }
        // TODO: Mads
        Expression::BinaryOp(binary_operation) => todo!(),
        // TODO: Kristoffer
        Expression::UnaryOp(unary_operation) => todo!(),
        // TODO: Thorbjoern
        Expression::FunctionCall(function_call) => todo!(),
        // TODO: Marck
        Expression::StructInit(struct_init) => todo!(),
        // TODO: Marck
        Expression::StructFieldAccess(struct_field_access) => todo!(),
        // TODO: Rasmus
        Expression::ArrayAccess(array_access) => {
            // Check if the array is declared in the variable table
            let array_type = check(&Expression::Variable(array_access.array_name.clone()), vtable, ftable)?;
            
            // Check if the type is an array
            if let Type::Array(element_type, _) = array_type {
                // Check if the index is an integer type
                let index_type = check(&array_access.index, vtable, ftable)?;
    
                // Check if index is any integer type
                match index_type {
                    Type::I8 | Type::I16 | Type::I32 | Type::I64 |
                    Type::U8 | Type::U16 | Type::U32 | Type::U64 => {
                        // Return the element type of the array
                        Ok(*element_type)
                    },
                    _ => {
                        Err(anyhow::anyhow!("Array index must be an integer, got {:?}", index_type))
                    }
                }
            } else {
                Err(anyhow::anyhow!("Array access on non-array type {:?}", array_type))
            }
        }

        // TODO: Mads
        Expression::ArrayLiteral(array_literal) => todo!(),
        // TODO: Mads
        Expression::TypeCast(type_cast) => todo!(),
    }
}
