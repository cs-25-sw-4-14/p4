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
        Expression::FunctionCall(function_call) => {

            // Check if the function is defined in the function table
            if let Some(func) = ftable.get(&function_call.name) {

                // Check if the function has the correct number of arguments
                if func.def.params.len() != function_call.args.len() {
                    return Err(anyhow::anyhow!("Function {} called with incorrect number of arguments", function_call.name));
                }
                

                let functype = func.def.return_type.clone().expect("REASON");
                println!("function checked {:?}",functype );
                Ok(functype)


            }else {
                Err(anyhow::anyhow!("function: {}, not defined", function_call.name))
            }


        }
        
        
        
        // TODO: Marck
        Expression::StructInit(struct_init) => todo!(),
        // TODO: Marck
        Expression::StructFieldAccess(struct_field_access) => todo!(),
        // TODO: Rasmus
        Expression::ArrayAccess(array_access) => todo!(),
        // TODO: Mads
        Expression::ArrayLiteral(array_literal) => todo!(),
        // TODO: Mads
        Expression::TypeCast(type_cast) => todo!(),
    }
}
