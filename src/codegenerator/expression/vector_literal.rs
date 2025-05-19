use super::CodeGenerator;
use crate::ast::VectorLiteral;
use anyhow::{Result, anyhow};
use inkwell::{AddressSpace, types::BasicTypeEnum, values::BasicValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_literal(
        &mut self,
        vector_literal: &VectorLiteral,
    ) -> Result<BasicValueEnum<'ctx>> {
        if vector_literal.elements.is_empty() {
            let null_ptr = self.context.ptr_type(AddressSpace::default()).const_null();
            let length = self.context.i64_type().const_int(0, false);
            self.heap_sizes.insert(null_ptr, length);
            return Ok(null_ptr.into());
        }
        let first_element = vector_literal.elements.first().unwrap();

        let first_value = self.compile_expression(first_element)?;
        let element_type = first_value.get_type();

        let element_size = (match element_type {
            BasicTypeEnum::FloatType(ft) => self.get_float_bit_width(ft)?,
            BasicTypeEnum::IntType(it) => it.get_bit_width(),
            _ => 64,
        } + 7)
            / 8;

        let nr_elements = vector_literal.elements.len() as u32;

        let total_size = element_size * nr_elements;

        let malloc_fn = self
            .module
            .get_function("malloc")
            .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

        let raw_ptr = self
            .builder
            .build_call(
                malloc_fn,
                &[self
                    .context
                    .i64_type()
                    .const_int(total_size as u64, false)
                    .into()],
                "malloc",
            )?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        let element_ptr_type = self.context.ptr_type(AddressSpace::default());

        let array_ptr = self
            .builder
            .build_pointer_cast(raw_ptr, element_ptr_type, "array_ptr")?;

        for (i, element_expr) in vector_literal.elements.iter().enumerate() {
            let value = self.compile_expression(element_expr)?;

            let idx = self.context.i64_type().const_int(i as u64, false);
            let element_ptr = unsafe {
                self.builder
                    .build_gep(element_type, array_ptr, &[idx], "element_ptr")
            }?;

            _ = self.builder.build_store(element_ptr, value);
        }

        let length = self
            .context
            .i64_type()
            .const_int(nr_elements as u64 + 1, false);
        self.heap_sizes.insert(array_ptr, length);

        Ok(array_ptr.into())
    }
}
