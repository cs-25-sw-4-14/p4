use super::CodeGenerator;
use crate::ast::VectorLiteral;
use anyhow::{Result, anyhow};
use inkwell::{
    AddressSpace,
    types::{BasicTypeEnum, StructType},
    values::{BasicValueEnum, PointerValue},
};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_literal(
        &mut self,
        vector_literal: &VectorLiteral,
    ) -> Result<BasicValueEnum<'ctx>> {
        if vector_literal.elements.is_empty() {
            let null_ptr = self.context.ptr_type(AddressSpace::default()).const_null();
            // track length = 0 so free() can see it
            self.heap_sizes
                .insert(null_ptr.into(), self.context.i64_type().const_zero());
            return Ok(null_ptr.into());
        }

        // --- 1) figure out element size & count ---
        let first_value = self.compile_expression(&vector_literal.elements[0])?;
        let element_type = first_value.get_type();
        let elem_bits = match element_type {
            BasicTypeEnum::FloatType(ft) => self.get_float_bit_width(ft)?,
            BasicTypeEnum::IntType(it) => it.get_bit_width(),
            _ => 64,
        };
        let element_size = (elem_bits + 7) / 8; // bytes per element
        let nr_elements = vector_literal.elements.len() as u64;

        // total heap size = element_size * nr_elements
        let total_size_bytes = element_size as u64 * nr_elements;

        // --- 2) call malloc(heap_size) ---
        let malloc_fn = self
            .module
            .get_function("malloc")
            .ok_or_else(|| anyhow!("malloc not declared"))?;
        let raw_ptr = self
            .builder
            .build_call(
                malloc_fn,
                &[self
                    .context
                    .i64_type()
                    .const_int(total_size_bytes, false)
                    .into()],
                "raw_data_alloc",
            )?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // track the length so free() knows how many elements to drop
        self.heap_sizes.insert(
            raw_ptr.into(),
            self.context.i64_type().const_int(nr_elements, false),
        );

        // --- 3) fill in the array ---
        let data_ptr = self.builder.build_pointer_cast(
            raw_ptr,
            self.context.ptr_type(AddressSpace::default()),
            "data_ptr",
        )?;
        for (i, expr) in vector_literal.elements.iter().enumerate() {
            let val = self.compile_expression(expr)?;
            let idx = self.context.i64_type().const_int(i as u64, false);
            let slot = unsafe {
                self.builder
                    .build_gep(element_type, data_ptr, &[idx], "slot")
            }?;
            self.builder.build_store(slot, val);
        }

        // --- 4) build the `{ ptr, i64 }` struct on the stack ---
        let header_bytes = self.context.i64_type().const_int(16, false);

        // get malloc again
        let header_malloc = self
            .builder
            .build_call(malloc_fn, &[header_bytes.into()], "vec_header_alloc")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // bitcast the raw pointer to a pointer-to-your-struct-type
        let ptr_ty = self.context.ptr_type(AddressSpace::default());
        let struct_type = self
            .context
            .struct_type(&[ptr_ty.into(), self.context.i64_type().into()], false);
        let header_ptr = self.builder.build_pointer_cast(
            header_malloc,
            struct_type.ptr_type(AddressSpace::default()),
            "vec_header_ptr",
        )?;

        // field 0 = data_ptr
        let data_field = self
            .builder
            .build_struct_gep(struct_type, header_ptr, 0, "data_field")?;
        self.builder.build_store(data_field, data_ptr);

        // field 1 = nr_elements
        let len_field = self
            .builder
            .build_struct_gep(struct_type, header_ptr, 1, "len_field")?;
        self.builder.build_store(
            len_field,
            self.context.i64_type().const_int(nr_elements, false),
        )?;

        // track header_ptr (not raw data) so free() can find the length
        self.heap_sizes.insert(
            header_ptr.into(),
            self.context.i64_type().const_int(nr_elements, false),
        );

        Ok(header_ptr.into())
    }
}
