use crate::{
    ast::{FunctionCall, Type},
    typechecker::Typechecker,
};
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_function_call(&mut self, function_call: &FunctionCall) -> Result<Type> {
        // Extract and clone everything needed from f_table before any mutable borrow of `self`
        let function_def = self
            .f_table
            .get(&function_call.name)
            .ok_or_else(|| anyhow::anyhow!("Function {} is not defined", function_call.name))?;

        let return_type = function_def.return_type.clone().ok_or_else(|| {
            anyhow::anyhow!(
                "Function {} has no return type and is therefore not an expression",
                function_def.name
            )
        })?;

        let param_types = function_def
            .params
            .iter()
            .map(|param| param.param_type.clone())
            .collect::<Vec<_>>();

        // Drop borrow of function_def here

        if param_types.len() != function_call.args.len() {
            return Err(anyhow::anyhow!(
                "Function {} called with incorrect number of arguments (expected {}, got {})",
                function_call.name,
                param_types.len(),
                function_call.args.len()
            ));
        }

        for (i, arg) in function_call.args.iter().enumerate() {
            let expected_type = &param_types[i];
            let actual_type = self.check_expression(arg)?;
            if expected_type != &actual_type {
                return Err(anyhow::anyhow!(
                    "Function {} argument {} has incorrect type (expected {:?}, got {:?})",
                    function_call.name,
                    i + 1,
                    expected_type,
                    actual_type
                ));
            }
        }

        Ok(return_type)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast::{Expression, FunctionCall, FunctionDef, Literal, Param, Type};
    use crate::debugger::Debugger;

    #[test]
    fn check_function_should_err_on_function_not_defined() {
        let expression = Expression::FunctionCall(FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);
        typechecker.check_expression(&expression).unwrap_err();
    }

    #[test]
    fn check_function_should_have_return_type() {
        let expression = Expression::FunctionCall(FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        typechecker.f_table.functions.insert(
            String::from("test"),
            FunctionDef {
                name: String::from("test"),
                params: vec![],
                return_type: None,
                body: vec![],
            },
        );

        typechecker.check_expression(&expression).unwrap_err();
    }

    #[test]
    fn check_function_args_and_params_are_same_length() {
        let expression = Expression::FunctionCall(FunctionCall {
            name: String::from("test"),
            args: vec![],
        });

        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        typechecker.f_table.functions.insert(
            String::from("test"),
            FunctionDef {
                name: String::from("test"),
                params: vec![Param {
                    name: String::from("a"),
                    param_type: Type::String,
                }],
                return_type: None,
                body: vec![],
            },
        );

        typechecker.check_expression(&expression).unwrap_err();
    }

    #[test]
    fn check_function_args_and_params_are_same_type() {
        let expression = Expression::FunctionCall(FunctionCall {
            name: String::from("test"),
            args: vec![Expression::Literal(Literal::Int(1))],
        });

        let debugger = Debugger::new(String::from(""));
        let mut typechecker = Typechecker::new(&debugger);

        typechecker.f_table.functions.insert(
            String::from("test"),
            FunctionDef {
                name: String::from("test"),
                params: vec![Param {
                    name: String::from("a"),
                    param_type: Type::I32,
                }],
                return_type: None,
                body: vec![],
            },
        );

        typechecker.check_expression(&expression).unwrap_err();
    }
}
