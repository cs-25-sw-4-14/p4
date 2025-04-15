use crate::{ast::{FunctionCall,FunctionDef, Type}, typechecker::{expression_check, function_table::{FTable, FunctionEntry}, struct_table::STable, variable_table::VTable}};
use anyhow::Result;

pub fn check(function_call: &FunctionCall, v_table: &VTable, f_table: &FTable, s_table: &STable) -> Result<Type> {
    
    let func = semantic_check_function_is_defined(function_call,f_table)?;

    let return_type = semantic_check_function_has_return_type(func)?;

    semantic_check_argument_length_are_equal(func,function_call)?;
    semantic_check_arguments_are_same_type(func, function_call, f_table, v_table, s_table)?;

    println!("Function {} called with correct argument types", function_call.name);
    Ok(return_type)
}

pub fn semantic_check_function_is_defined<'a>(function_call: &'a FunctionCall, f_table: &'a FTable) -> Result<&'a FunctionDef> {
    let func = f_table.get(&function_call.name)
        .ok_or_else(|| anyhow::anyhow!("function: {}, not defined", function_call.name))?;
    Ok(func.def)
}

pub fn semantic_check_function_has_return_type<'a>(function: &'a FunctionDef) -> Result<Type>{
    let return_type = function.return_type.clone()
        .ok_or_else(|| anyhow::anyhow!("Function {} has no return type and is therefore not an expression", function.def.name))?;
    Ok(return_type)
}

pub fn semantic_check_argument_length_are_equal<'a>(function: &'a FunctionDef, function_call: &'a FunctionCall) -> Result<()>{
    if function.params.len() != function_call.args.len(){
        return Err(anyhow::anyhow!("Function {} called with incorrect number of arguments", function_call.name));
    }
    Ok(())
}

pub fn semantic_check_arguments_are_same_type<'a>(function: &'a FunctionDef, function_call: &'a FunctionCall, f_table: &'a FTable, v_table: &'a VTable, s_table: &'a STable )-> Result<()>{

    for i in 0..function.params.len() {
        let expected_type = function.params[i].param_type.clone();
        let actual_type = expression_check::check(&function_call.args[i], v_table, f_table, s_table)?;
            if expected_type != actual_type {
                return Err(anyhow::anyhow!("Function {} called with incorrect argument types", function_call.name));
        }
        }
    Ok(())
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

        let v_table = VTable::new();
        let f_table = FTable::new();
        let s_table = STable::new();

        expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap_err();
    }

    #[test]
    fn check_function_should_have_return_type() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let v_table = VTable::new();
        let mut f_table = FTable::new();
        let s_table = STable::new();

        f_table.functions.insert(String::from("test"), FunctionEntry { 
            def: FunctionDef {
                name: String::from("test"),
                params: vec![],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });
        expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap_err();
    }

    #[test]
    fn check_function_args_and_params_are_same_length() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let v_table = VTable::new();
        let mut f_table = FTable::new();
        let s_table = STable::new();

        f_table.functions.insert(String::from("test"), FunctionEntry { 
            def: FunctionDef {
                name: String::from("test"),
                params: vec![ Param { name: String::from("a"), param_type: Type::String }],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });

        expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap_err();
    }

    #[test]
    fn check_function_args_and_params_are_same_type() {
        let expression = Expression::FunctionCall( FunctionCall {
            name: String::from("test"),
            args: vec![Expression::Literal(Literal::Int(1))],
        });

        let v_table = VTable::new();
        let mut f_table = FTable::new();
        let s_table = STable::new();

        f_table.functions.insert(String::from("test"), FunctionEntry { 
            def: FunctionDef {
                name: String::from("test"),
                params: vec![ Param { name: String::from("a"), param_type: Type::I32 }],
                return_type: None,
                body: vec![],
            },
            vtable: VTable::new(),
        });

        expression_check::check(&expression, &v_table, &f_table, &s_table).unwrap_err();
    }
}



