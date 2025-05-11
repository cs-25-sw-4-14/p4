use super::CodeGenerator;
use inkwell::{values::BasicValueEnum, AddressSpace};
use anyhow::Result;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_null_ptr(&self) -> Result<BasicValueEnum<'ctx>> {
        let i8_ptr_type = self.context.ptr_type(AddressSpace::default());
        let null_ptr = i8_ptr_type.const_null();
        Ok(null_ptr.into())
    }
}
