use crate::{
    ast::{FunctionCall, Type},
    typechecker::Typechecker,
};
use anyhow::Result;

impl Typechecker<'_> {
    /// Typechecks a function call expression.
    /// Ensures the function exists, the argument count matches, and each argument type matches the function parameter type.
    /// Returns the function's return type if successful.
    pub fn check_function_call(&mut self, function_call: &FunctionCall) -> Result<Type> {
        // Look up the function definition by name in the function table (f_table).
        // Clone the function definition to avoid borrow conflicts when checking arguments later.
        let function_def = self
            .f_table
            .get(&function_call.name)
            .ok_or_else(|| anyhow::anyhow!("Function {} is not defined", function_call.name))?;

        // Extract the return type of the function. If the function has no return type, it cannot be used as an expression.
        let return_type = function_def.return_type.clone().ok_or_else(|| {
            anyhow::anyhow!(
                "Function {} has no return type and is therefore not an expression",
                function_def.name
            )
        })?;

        // Collect all parameter types in order to compare them with the types of arguments passed in the call.
        let param_types = function_def
            .params
            .iter()
            .map(|param| param.param_type.clone())
            .collect::<Vec<_>>();

        // At this point, the borrow on function_def ends so we can freely use `self` again.

        // Check that the number of arguments matches the number of parameters.
        if param_types.len() != function_call.args.len() {
            return Err(anyhow::anyhow!(
                "Function {} called with incorrect number of arguments (expected {}, got {})",
                function_call.name,
                param_types.len(),
                function_call.args.len()
            ));
        }

        // Check each argument's type against the expected parameter type.
        for (i, arg) in function_call.args.iter().enumerate() {
            let expected_type = &param_types[i];
            // Recursively check the argument expression to get its type.
            let actual_type = self.check_expression(arg)?;

            // If types don't match, return a descriptive error.
            if expected_type != &actual_type {
                return Err(anyhow::anyhow!(
                    "Function {} argument {} has incorrect type (expected {:?}, got {:?})",
                    function_call.name,
                    i + 1, // 1-based argument index for better readability
                    expected_type,
                    actual_type
                ));
            }
        }

        // If all checks pass, return the function's return type.
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
