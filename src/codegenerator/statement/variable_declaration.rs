use super::CodeGenerator;
use crate::ast::VariableDeclaration;
use anyhow::{Result, anyhow};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_variable_declaration(&mut self, decl: &VariableDeclaration) -> Result<()> {
        let value = self.compile_expression(&decl.value)?;
    
        // Use get_basic_type to convert the variable type
        let llvm_var_type = self.get_basic_type(&decl.var_type)?;
    
        // Allocate space for the variable on the stack
        let entry_block = self.function.get_first_basic_block()
            .ok_or_else(|| anyhow!("Function has no entry block"))?;
    
        let builder = self.context.create_builder();
        if let Some(first_instruction) = entry_block.get_first_instruction() {
            builder.position_at(entry_block, &first_instruction);
        } else {
            builder.position_at_end(entry_block);
        }
    
        let alloca = builder.build_alloca(llvm_var_type, &decl.name);
    
        // Check if the variable is mutable (using Option<()> to check)
        let is_mutable = decl.mutable.is_some(); // Mutable if Some(()), not mutable if None
    
        // Store the value into the alloca using the main builder
        if value.get_type() != llvm_var_type {
            return Err(anyhow!("Type mismatch: Initializer type {:?} does not match variable type {:?} ({:?}) for '{}'",
                value.get_type(), decl.var_type, llvm_var_type, decl.name));
        }
        self.builder.build_store(alloca, value);
    
        // Register the variable in the symbol table with its mutability flag
        self.variables.insert(decl.name.clone(), (alloca, is_mutable));
    
        Ok(())
    }    
    
}