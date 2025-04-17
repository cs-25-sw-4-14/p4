use crate::ast::Expression;
use super::CodeGenerator;
use anyhow::Result;
use inkwell::values::BasicValueEnum;

pub mod variable;
pub mod literal;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_expression(&mut self, expr: &Expression) -> Result<BasicValueEnum<'ctx>> {
        match expr {
            Expression::Literal(literal) => self.compile_literal(literal),  
            Expression::Variable(variable) => self.compile_variable(variable),
            // TODO: Marc
            Expression::BinaryOp(binary_operation) => todo!(),
            Expression::UnaryOp(unary_operation) => todo!(),

            Expression::FunctionCall(function_call) => {
                // Look up the function in the module
                let function = match self.module.get_function(&function_call.name) {
                    Some(f) => f,
                    None => return Err(anyhow::anyhow!("Function '{}' not found", function_call.name)),
                };
                
                // Compile each argument expression
                let mut arg_values = Vec::with_capacity(function_call.args.len());
                for arg_expr in &function_call.args {
                    arg_values.push(self.compile_expression(arg_expr)?);
                }
                
                // Convert the arguments to LLVM's expected format (BasicMetadataValueEnum)
                let args: Vec<inkwell::values::BasicMetadataValueEnum> = arg_values
                    .iter()
                    .map(|val| (*val).into())
                    .collect();
                
                // Build the function call
                let call_site_value = self.builder
                    .build_call(function, &args, &format!("call_{}", function_call.name))?
                    .try_as_basic_value()
                    .left();
                
                call_site_value
                    .ok_or_else(|| anyhow::anyhow!("Function call did not return a value"))
            },

            Expression::StructInit(struct_init) => todo!(),
            Expression::StructFieldAccess(struct_field_access) => todo!(),
            Expression::ArrayAccess(array_access) => todo!(),
            Expression::ArrayLiteral(array_literal) => todo!(),
            Expression::TypeCast(type_cast) => todo!(),
        }
    }
}
