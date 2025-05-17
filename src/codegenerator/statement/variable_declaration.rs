use super::CodeGenerator;
use crate::ast::{Type, VariableDeclaration};
use anyhow::{Result, anyhow};

impl CodeGenerator<'_> {
    pub fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        let value = self.compile_expression(&decl.value)?;

        let var_name = &decl.name;

        let llvm_var_type = match &decl.var_type {
            Type::Vector(element_type, _initial_size) => {
                let vec_val = value.clone().into_vector_value();
                let nr_elements = vec_val.get_type().get_size();
                let var_type = &Type::Vector(element_type.clone(), nr_elements);
                self.get_llvm_type(var_type)
            }
            _ => {
                self.get_llvm_type(&decl.var_type)
            }
        }?;

        let var_ptr = self
            .builder
            .build_alloca(llvm_var_type, var_name)
            .map_err(|e| {
                anyhow!(
                    "Failed to allocate memory for variable '{}': {}",
                    var_name,
                    e
                )
            })?;

        self.builder.build_store(var_ptr, value)?;

        self.variables
            .insert(var_name.clone(), (llvm_var_type, var_ptr));

        Ok(())
    }
}
