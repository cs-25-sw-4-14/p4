use super::CodeGenerator;
use crate::ast::StructDef;
use anyhow::Result;
use inkwell::types::BasicTypeEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct(&mut self, struct_def: &StructDef) -> Result<()> {
        // Store the program for reference if not already stored
        if self.program.is_none() {
            return Err(anyhow::anyhow!("Program not loaded"));
        }

        // Create an opaque struct type first (with no body defined yet)
        let struct_type = self.context.opaque_struct_type(&struct_def.name);
        
        // Convert field types to LLVM types
        let field_types: Result<Vec<BasicTypeEnum<'ctx>>> = struct_def
            .fields
            .iter()
            .map(|field| self.get_llvm_type(&field.field_type))
            .collect();

        // Set the struct body with the field types
        let field_types = field_types?;
        struct_type.set_body(&field_types, false);

        Ok(())
    }
}