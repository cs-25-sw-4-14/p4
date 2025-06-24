use super::CodeGenerator;
use anyhow::Result;
use inkwell::{AddressSpace, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a null pointer of type `i8*` (pointer to byte) and returns it as a BasicValueEnum.
    ///
    /// # Returns
    /// * `Result<BasicValueEnum>` - LLVM representation of a null pointer.
    pub fn compile_null_ptr(&self) -> Result<BasicValueEnum<'ctx>> {
        // Get the LLVM pointer type to i8 (byte pointer), in the default address space (usually 0).
        let i8_ptr_type = self.context.ptr_type(AddressSpace::default());

        // Create a constant null pointer of type i8*
        let null_ptr = i8_ptr_type.const_null();

        // Return the null pointer wrapped as a BasicValueEnum
        Ok(null_ptr.into())
    }
}
