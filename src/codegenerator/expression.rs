use super::CodeGenerator;
use crate::ast::Expression;
use anyhow::Result;
use anyhow::anyhow;
use inkwell::values::BasicValueEnum;

pub mod array_access;
pub mod array_literal;
pub mod binary_op;
pub mod function_call;
pub mod literal;
pub mod null;
pub mod read_file;
pub mod reference;
pub mod struct_field_access;
pub mod struct_init;
pub mod type_cast;
pub mod unary_op;
pub mod variable;
pub mod vector_access;
pub mod vector_literal;
pub mod write_file;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_expression(&mut self, expr: &Expression) -> Result<BasicValueEnum<'ctx>> {
        match expr {
            Expression::Reference(name, _) => self.compile_reference(name),
            Expression::Dereference(name) => {
                let var = self
                    .variables
                    .get(name)
                    .ok_or_else(|| anyhow!("Undefined variable: {}", name))?;
                let loaded = self.builder.build_load(var.0, var.1, name);
                Ok(loaded?)
            }
            Expression::Literal(literal) => self.compile_literal(literal),
            Expression::Variable(variable) => self.compile_variable(variable),
            Expression::BinaryOp(binary_operation) => self.compile_binary_op(binary_operation),
            Expression::UnaryOp(unary_operation) => self.compile_unary_op(unary_operation),
            Expression::FunctionCall(function_call) => {
                self.compile_function_call_expr(function_call)
            }
            Expression::VectorAccess(vector_access) => self.compile_vector_access(vector_access),
            Expression::VectorLiteral(vector_literal) => {
                self.compile_vector_literal(vector_literal)
            }
            Expression::StructInit(struct_init) => self.compile_struct_init(struct_init),
            Expression::StructFieldAccess(struct_field_access) => {
                self.compile_struct_field_access(struct_field_access)
            }
            Expression::ArrayAccess(array_access) => self.compile_array_access(array_access),
            Expression::ArrayLiteral(array_literal) => self.compile_array_literal(array_literal),
            Expression::TypeCast(type_cast) => {
                let value = self.compile_type_cast(type_cast)?;
                self.last_value = Some(value);
                Ok(value)
            }
            Expression::Write(write_node) => self.compile_write_file(write_node),
            Expression::Read(read_node) => self.compile_read_file(read_node),
            Expression::Null => self.compile_null_ptr(),
        }
    }
}
