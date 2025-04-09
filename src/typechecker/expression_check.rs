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
                //the return type of the function
                let functype = func.def.return_type.clone().unwrap_or(Type::Void);



                // check if the number of arguments is correct
                if func.def.params.len() != function_call.args.len() {
                    return Err(anyhow::anyhow!("Function {} called with incorrect number of arguments", function_call.name));
                }
                

                // get all the argument types from ftable
                let fTableArgTypes = func.def.params.iter()
                .map(|param| param.param_type.clone())
                .collect::<Vec<Type>>();      

                // get all the argument types from the function call
                let actualArgTypes = function_call.args.iter()
                .map(|arg| check(arg, vtable, ftable))
                .collect::<Result<Vec<Type>>>()?;
                
                //check if the types are the same
                for i in 0..fTableArgTypes.len() {
                    if fTableArgTypes[i] != actualArgTypes[i] {
                        return Err(anyhow::anyhow!("Function {} called with incorrect argument types", function_call.name));
                    }
                }
            
            
            println!("function checked {}, with return type {}", function_call.name, functype);
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
