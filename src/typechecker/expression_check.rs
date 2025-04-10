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
            // Kontroller at array-udtrykket har en array-type
            let array_type = check(&array_access.array, vtable, ftable)?;
            
            // Kontroller at array_type faktisk er et array
            if let Type::Array(element_type) = array_type {
                // Kontroller at indekset er et heltal
                let index_type = check(&array_access.index, vtable, ftable)?;
                
                if matches!(index_type, Type::Int) {
                    // Returnér element-typen
                    Ok(*element_type)
                } else {
                    Err(anyhow::anyhow!("Array index must be an integer, got {:?}", index_type))
                }
            } else {
                Err(anyhow::anyhow!("Cannot index into non-array type {:?}", array_type))
            }
        }
    

        // TODO: Mads
        Expression::ArrayLiteral(array_literal) => todo!(),
        // TODO: Mads
        Expression::TypeCast(type_cast) => todo!(),
    }
}
