use crate::ast::FunctionCall;
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_function_call_stmt(&mut self, declaration: &FunctionCall) -> Result<()> {
        // Step 1: Extract the necessary data from f_table into local variables
        let (return_type, param_types) = {
            let function_def = self
                .f_table
                .get(&declaration.name)
                .ok_or_else(|| anyhow::anyhow!("Function {} is not defined", declaration.name))?;

            // Copy what we need
            (
                function_def.return_type.clone(),
                function_def
                    .params
                    .iter()
                    .map(|p| p.param_type.clone())
                    .collect::<Vec<_>>(),
            )
        };

        // Step 2: Validate return type is void
        if return_type.is_some() {
            return Err(anyhow::anyhow!(
                "Function {} is not void (has a return type)",
                declaration.name
            ));
        }

        // Step 3: Check number of arguments
        if param_types.len() != declaration.args.len() {
            return Err(anyhow::anyhow!(
                "Function {} has {} parameters, but {} arguments were provided",
                declaration.name,
                param_types.len(),
                declaration.args.len()
            ));
        }

        // Step 4: Check argument types
        for (i, arg) in declaration.args.iter().enumerate() {
            let arg_type = self.check_expression(arg)?;
            if arg_type != param_types[i] {
                return Err(anyhow::anyhow!(
                    "Type mismatch for argument {}: expected {:?}, found {:?}",
                    i + 1,
                    param_types[i],
                    arg_type
                ));
            }
        }

        Ok(())
    }
}
