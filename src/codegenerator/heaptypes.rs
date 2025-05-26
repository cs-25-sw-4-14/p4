use anyhow::Error;
use inkwell::{AddressSpace, values::PointerValue};

use super::CodeGenerator;

pub struct HeapString {
    pub length: i64,
    pub string: String,
}

pub struct HeapVector<T> {
    pub size: i64,
    pub data: [T; 0],
}

impl HeapString {
    pub fn new(string: &String) -> Self {
        let length = string.len() as i64;
        HeapString {
            length,
            string: string.clone(),
        }
    }
}

impl HeapVector<i64> {
    pub fn new(size: i64) -> Self {
        HeapVector { size, data: [] }
    }
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn get_string_length(
        &self,
        string_ptr: inkwell::values::PointerValue<'ctx>,
    ) -> Result<inkwell::values::IntValue<'ctx>, Error> {
        let string_struct_type = self.context.struct_type(
            &[
                self.context.i64_type().into(),              // length
                self.context.i8_type().array_type(0).into(), // data
            ],
            false,
        );

        let string_struct_ptr = self.builder.build_pointer_cast(
            string_ptr,
            self.context.ptr_type(AddressSpace::default()),
            "string_struct_ptr",
        )?;

        let length_ptr = self.builder.build_struct_gep(
            string_struct_type,
            string_struct_ptr,
            0,
            "length_ptr",
        )?;

        Ok(self
            .builder
            .build_load(self.context.i64_type(), length_ptr, "string_length")?
            .into_int_value())
    }

    pub fn get_string_data_ptr(
        &self,
        string_ptr: inkwell::values::PointerValue<'ctx>,
    ) -> Result<inkwell::values::PointerValue<'ctx>, Error> {
        let string_struct_type = self.context.struct_type(
            &[
                self.context.i64_type().into(),              // length
                self.context.i8_type().array_type(0).into(), // data
            ],
            false,
        );

        let string_struct_ptr = self.builder.build_pointer_cast(
            string_ptr,
            self.context.ptr_type(AddressSpace::default()),
            "string_struct_ptr",
        )?;

        let data_ptr =
            self.builder
                .build_struct_gep(string_struct_type, string_struct_ptr, 1, "data_ptr")?;

        Ok(data_ptr)
    }

    pub fn get_string_struct_type(&self) -> inkwell::types::StructType<'ctx> {
        self.context.struct_type(
            &[
                self.context.i64_type().into(),              // length
                self.context.i8_type().array_type(0).into(), // data (flexible array)
            ],
            false,
        )
    }

    pub fn get_vector_data_ptr_from_vector(
        &self,
        vector_ptr: &PointerValue<'ctx>,
    ) -> Result<inkwell::values::PointerValue<'ctx>, Error> {
        // Cast to i8* for pointer arithmetic
        let data_ptr = unsafe {
            self.builder.build_gep(
                self.context.i8_type(),
                *vector_ptr,
                &[self.context.i64_type().const_int(8, false)],
                "data_ptr",
            )?
        };

        // Cast back to the appropriate element type pointer
        Ok(data_ptr)
    }
}
