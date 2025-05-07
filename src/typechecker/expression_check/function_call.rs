use crate::{
    ast::{FunctionCall, FunctionDef, Type},
    typechecker::Typechecker,
};
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_function_call(&self, function_call: &FunctionCall) -> Result<Type> {
        let func = self.semantic_check_function_is_defined(function_call)?;

        let return_type = Self::semantic_check_function_has_return_type(func)?;

        Self::semantic_check_argument_length_are_equal(func, function_call)?;
        self.semantic_check_arguments_are_same_type(func, function_call)?;

        println!(
            "Function {} called with correct argument types",
            function_call.name
        );
        Ok(return_type)
    }

    pub fn semantic_check_function_is_defined(
        &self,
        function_call: &FunctionCall,
    ) -> Result<&FunctionDef> {
        let function_def = self
            .f_table
            .get(&function_call.name)
            .ok_or_else(|| anyhow::anyhow!("function: {}, not defined", function_call.name))?;
        Ok(function_def)
    }

    pub fn semantic_check_function_has_return_type(function: &FunctionDef) -> Result<Type> {
        let return_type = function.return_type.clone().ok_or_else(|| {
            anyhow::anyhow!(
                "Function {} has no return type and is therefore not an expression",
                function.name
            )
        })?;
        Ok(return_type)
    }

    pub fn semantic_check_argument_length_are_equal(
        function: &FunctionDef,
        function_call: &FunctionCall,
    ) -> Result<()> {
        if function.params.len() != function_call.args.len() {
            return Err(anyhow::anyhow!(
                "Function {} called with incorrect number of arguments",
                function_call.name
            ));
        }
        Ok(())
    }

    pub fn semantic_check_arguments_are_same_type(
        &self,
        function: &FunctionDef,
        function_call: &FunctionCall,
    ) -> Result<()> {
        for i in 0..function.params.len() {
            let expected_type = function.params[i].param_type.clone();
            let actual_type = self.check_expression(&function_call.args[i])?;
            if expected_type != actual_type {
                return Err(anyhow::anyhow!(
                    "Function {} called with incorrect argument types",
                    function_call.name
                ));
            }
        }

        Ok(())
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
        let typechecker = Typechecker::new(&debugger);
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
