use super::CodeGenerator;
use crate::ast::{FunctionDef, ReturnStatement, Statement, Type};
use anyhow::Result;
use inkwell::types::{BasicType, BasicTypeEnum, BasicMetadataTypeEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_function(&mut self, function_def: &FunctionDef) -> Result<()> {
        // Create function parameter types
        let param_types: Vec<BasicTypeEnum<'ctx>> = function_def
            .params
            .iter()
            .map(|param| self.get_llvm_type(&param.param_type))
            .collect::<Result<Vec<_>>>()?;

        // Create function return type
        let return_type = match &function_def.return_type {
            Some(ret_type) => {
                if let Type::Struct(_) = ret_type {
                    return Err(anyhow::anyhow!("Returning struct by value not supported yet"));
                }
                Some(self.get_llvm_type(ret_type)?)
            }
            None => None, // Void return type
        };

// Convert to BasicMetadataTypeEnum for fn_type
        let param_types_meta: Vec<BasicMetadataTypeEnum<'ctx>> = param_types
            .iter()
            .map(|ty| (*ty).into())
            .collect();

        // Create function type
        let fn_type = match return_type {
            Some(ret_type) => ret_type.fn_type(&param_types_meta, false),
            None => self.context.void_type().fn_type(&param_types_meta, false),
        };

        // Create the function
        let function = self.module.add_function(&function_def.name, fn_type, None);

        // Create a new basic block for the function
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        // Save the current state
        let prev_variables = self.variables.clone();
        self.variables.clear(); // Clear variables for new function scope

        // Allocate space for parameters and store their values
        for (i, param) in function_def.params.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).ok_or_else(|| {
                anyhow::anyhow!("Failed to get parameter {} from function", param.name)
            })?;

            // Allocate space for the parameter
            let param_type = self.get_llvm_type(&param.param_type)?;
            let alloca = self.builder.build_alloca(param_type, &param.name)?;
            
            // Store the parameter value
            self.builder.build_store(alloca, param_value)?;
            
            // Add to variables map
            self.variables.insert(param.name.clone(), (param_type, alloca));
        }

        // Compile the function body
        for statement in &function_def.body {
            self.compile_statement(statement)?;
        }

        // Add a return statement if the function doesn't have one and has void return type
        if function_def.return_type.is_none() {
            // Check if the function ends with a return statement
            let has_return = function_def.body.last().map_or(false, |stmt| {
                matches!(stmt, Statement::Return(_))
            });

            if !has_return {
                self.builder.build_return(None)?;
            }
        }

        // Verify the function
        if function.verify(true) {
            // Restore the previous state
            self.variables = prev_variables;
            Ok(())
        } else {
            // Restore the previous state
            self.variables = prev_variables;
            
            // Delete the invalid function
            unsafe { function.delete(); }
            Err(anyhow::anyhow!("Invalid function: {}", function_def.name))
        }
    }

    pub fn compile_return_statement(&mut self, ret_stmt: &ReturnStatement) -> Result<()> {
        match &ret_stmt.value {
            Some(expr) => {
                let return_value = self.compile_expression(expr)?;
                self.builder.build_return(Some(&return_value))?;
            }
            None => {
                self.builder.build_return(None)?;
            }
        }
        Ok(())
    }

    // Using the common get_llvm_type method from the main CodeGenerator implementation
}