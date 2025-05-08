use crate::ast::FunctionCall;
use crate::typechecker::Typechecker;
use anyhow::Result;

impl Typechecker<'_> {
    pub fn check_function_call_stmt(&self, declaration: &FunctionCall) -> Result<()> {
        // Check if the function is defined
        if let Some(function_def) = self.f_table.get(&declaration.name) {
            // Check if the function is void
            if function_def.return_type.is_some() {
                return Err(anyhow::anyhow!(
                    "Function {} is not void (has a return type)",
                    declaration.name
                ));
            }
            // Check if the number of arguments matches
            if function_def.params.len() != declaration.args.len() {
                return Err(anyhow::anyhow!(
                    "Function {} has {} parameters, but {} arguments were provided",
                    declaration.name,
                    function_def.params.len(),
                    declaration.args.len()
                ));
            }
            // Check if the types of the arguments match
            for (i, arg) in declaration.args.iter().enumerate() {
                let arg_type = self.check_expression(arg)?;
                let param_type = &function_def.params[i].param_type;
                if arg_type != *param_type {
                    return Err(anyhow::anyhow!(
                        "Type mismatch for argument {}: expected {:?}, found {:?}",
                        i + 1,
                        param_type,
                        arg_type
                    ));
                }
            }
        } else {
            return Err(anyhow::anyhow!(
                "Function {} is not defined",
                declaration.name
            ));
        }

        Ok(())
    }
}
