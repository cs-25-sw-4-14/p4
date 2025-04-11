use crate::{ast::{FunctionCall, Literal, Type}, typechecker::{expression_check, function_table::{self, Ftable}, variable_table::VTable}};
use anyhow::Result;

pub fn check(function_call: &FunctionCall, vtable: &VTable, ftable: &Ftable) -> Result<Type> {
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
        let actual_type = expression_check::check(&function_call.args[i], vtable, ftable)?;
            if expected_type != actual_type {
                return Err(anyhow::anyhow!("Function {} called with incorrect argument types", function_call.name));
        }
        }

    println!("Function {} called with correct argument types", function_call.name);
    Ok(return_type)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{Expression, FunctionDef, Literal, Param, Type, FunctionCall};
    use crate::typechecker::function_table::FunctionEntry;
    use crate::typechecker::expression_check;
    use crate::typechecker::variable_table::VTable;

    #[test]
    fn check_function_should_err_on_function_not_defined() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let vtable = VTable::new();
        let ftable = Ftable::new();

        expression_check::check(&expression, &vtable, &ftable).unwrap_err();
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

        expression_check::check(&expression, &vtable, &ftable).unwrap_err();
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
                params: vec![ Param { name: String::from("a"), param_type: Type::String }],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });

        expression_check::check(&expression, &vtable, &ftable).unwrap_err();
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

        expression_check::check(&expression, &vtable, &ftable).unwrap_err();
    }
}



