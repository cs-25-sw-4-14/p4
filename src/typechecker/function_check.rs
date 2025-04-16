use super::{
    function_table::FTable, statement_check, struct_table::STable, variable_table::VTable,
};
use crate::ast::{FunctionDef, ReturnStatement, Statement, Type};
use anyhow::{Result, anyhow};
use std::collections::HashSet;

pub fn check(function: &FunctionDef, f_table: &mut FTable, s_table: &STable) -> Result<()> {
    // check if function exists in the function table
    if f_table.get(&function.name).is_none() {
        return Err(anyhow!(
            "Function {} not defined in function table",
            function.name
        ));
    }

    let v_table = &mut f_table.get_mut(&function.name).unwrap().vtable;

    //check for duplicate parameter names
    check_duplicate_params(function)?;

    // add parameters to the variable table
    for param in &function.params {
        v_table.variables.insert(
            param.name.clone(),
            crate::ast::VariableDeclaration {
                trivial: None,
                mutable: None,
                var_type: param.param_type.clone(),
                name: param.name.clone(),
                value: crate::ast::Expression::Literal(crate::ast::Literal::Int(0)), // bare noget 🤷‍♂️ ligegyldigt!
            },
        );
    }

    // type-check the function body
    check_function_body(function, v_table, f_table, s_table)?;

    // check return statements
    check_return_statements(function, &v_table, f_table, s_table)?;

    Ok(())
}

fn check_duplicate_params(function: &FunctionDef) -> Result<()> {
    let mut param_names = HashSet::new();

    for param in &function.params {
        if !param_names.insert(&param.name) {
            return Err(anyhow!(
                "Duplicate parameter name: {} in function {}",
                param.name,
                function.name
            ));
        }
    }

    Ok(())
}

fn check_function_body(
    function: &FunctionDef,
    v_table: &mut VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<()> {
    for statement in &function.body {
        statement_check::var_declaration(statement, v_table, f_table, s_table)?;
    }

    Ok(())
}

fn check_return_statements(
    function: &FunctionDef,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<()> {
    let return_type = &function.return_type;
    let mut has_return = false;

    for statement in &function.body {
        if let Statement::Return(ret) = statement {
            has_return = true;
            check_return_type(ret, return_type, v_table, f_table, s_table)?;
        }
    }

    //check if function with non-void return type has at least one return statement
    if return_type.is_some() && !has_return {
        return Err(anyhow!(
            "Function {} has return type {:?} but no return statement",
            function.name,
            return_type
        ));
    }

    Ok(())
}

fn check_return_type(
    ret: &ReturnStatement,
    expected_type: &Option<Type>,
    v_table: &VTable,
    f_table: &FTable,
    s_table: &STable,
) -> Result<()> {
    match (expected_type, &ret.value) {
        // Case 1: Function returns void, return statement should be empty
        (None, Some(_)) => Err(anyhow!("Function has void return type but returns a value")),

        // Case 2: Function returns non-void, return statement should have matching type
        (Some(expected), Some(expr)) => {
            let actual_type = super::expression_check::check(expr, v_table, f_table, s_table)?;
            if *expected != actual_type {
                Err(anyhow!(
                    "Return type mismatch: expected {:?}, found {:?}",
                    expected,
                    actual_type
                ))
            } else {
                Ok(())
            }
        }

        // Case 3: Function returns non-void, but return statement is empty
        (Some(expected), None) => Err(anyhow!(
            "Function has return type {:?} but return statement is empty",
            expected
        )),

        // Case 4: Function returns void, and return statement is empty (valid)
        (None, None) => Ok(()),
    }
}
