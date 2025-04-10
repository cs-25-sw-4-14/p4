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
            let func = ftable.get(&function_call.name)
                .ok_or_else(|| anyhow::anyhow!("function: {}, not defined", function_call.name))?;

            let return_type = func.def.return_type.clone()
                .ok_or_else(|| anyhow::anyhow!("Function {} has no return type and is therefore not an expression", function_call.name))?;

            // check if the number of arguments is correct
            if func.def.params.len() != function_call.args.len() {
                return Err(anyhow::anyhow!("Function {} called with incorrect number of arguments", function_call.name));
            }

             //check if the types are the same
             for i in 0..func.def.params.len() {
                let expected_type = func.def.params[i].param_type.clone();
                let actual_type = check(&function_call.args[i], vtable, ftable)?;
                 if expected_type != actual_type {
                     return Err(anyhow::anyhow!("Function {} called with incorrect argument types", function_call.name));
                }
             }

            println!("Function {} called with correct argument types", function_call.name);
            Ok(return_type)
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

#[cfg(test)]
pub mod function_call_tests {
    use super::*;
    use crate::ast::{Expression, FunctionDef, Literal, Param, Type, FunctionCall};
    use crate::lossy::ExpressionParser;
    use crate::typechecker::function_table::FunctionEntry;

    #[test]
    fn check_function_should_err_on_function_not_defined() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let vtable = VTable::new();
        let ftable = Ftable::new();

        check(&expression, &vtable, &ftable).unwrap_err();
    }

    #[test]
    fn check_function_should_have_return_type() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let vtable = VTable::new();
        let mut ftable = Ftable::new();

        ftable.functions.insert(String::from("test"), FunctionEntry { 
            def: FunctionDef {
                name: String::from("test"),
                params: vec![],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });

        check(&expression, &vtable, &ftable).unwrap_err();
    }

    #[test]
    fn check_function_args_and_params_are_same_length() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![],
        });
        let vtable = VTable::new();
        let mut ftable = Ftable::new();

        ftable.functions.insert(String::from("test"), FunctionEntry { 
            def: FunctionDef {
                name: String::from("test"),
                params: vec![ Param { name: String::from("a"), param_type: Type::I32 }],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });

        check(&expression, &vtable, &ftable).unwrap_err();
    }

    #[test]
    fn check_function_args_and_params_are_same_type() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![Expression::Literal(Literal::Int(1))],
        });

        let vtable = VTable::new();
        let mut ftable = Ftable::new();

        ftable.functions.insert(String::from("test"), FunctionEntry { 
            def: FunctionDef {
                name: String::from("test"),
                params: vec![ Param { name: String::from("a"), param_type: Type::I32 }],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });

        check(&expression, &vtable, &ftable).unwrap_err();
    }
}

