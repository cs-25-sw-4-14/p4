use super::CodeGenerator;
use crate::ast::{Type, VariableDeclaration};
use anyhow::Result;
use inkwell::{AddressSpace, types::BasicTypeEnum};

impl CodeGenerator<'_> {
    pub fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        let value = self.compile_expression(&decl.value)?;
        let var_name = &decl.name;

        match &decl.var_type {
            Type::Vector(element_type, _) => {
                let vector_ptr = value.into_pointer_value();

                let var_ptr = self
                    .builder
                    .build_alloca(self.context.ptr_type(AddressSpace::default()), var_name)?;

                self.builder.build_store(var_ptr, vector_ptr)?;

                let size_type = self.context.i64_type();
                let llvm_element_type = self.get_llvm_type(element_type)?;
                let vector_struct_type = self
                    .context
                    .struct_type(&[llvm_element_type.into(), size_type.into()], false);

                let llvm_struct_type = BasicTypeEnum::StructType(vector_struct_type);

                self.variables
                    .insert(var_name.clone(), (llvm_struct_type, var_ptr));

                return Ok(());
            }
            _ => {
                let llvm_var_type = self.get_llvm_type(&decl.var_type)?;
                let var_ptr = self.builder.build_alloca(llvm_var_type, var_name)?;
                self.builder.build_store(var_ptr, value)?;
                self.variables
                    .insert(var_name.clone(), (llvm_var_type, var_ptr));

                Ok(())
            }
        }
    }
}
