use crate::ast::VectorInsert;
use crate::codegenerator::BasicValueEnum;
use crate::codegenerator::CodeGenerator;
use anyhow::Result;
use anyhow::anyhow;
use inkwell::AddressSpace;
use inkwell::types::BasicTypeEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_insert(&mut self, vector_insert: &VectorInsert) -> Result<()> {
        let vector_name = &vector_insert.vector_name;
        let insert_index = &vector_insert.index;
        let insert_value = &vector_insert.value;
        let value_expr = self.compile_expression(insert_value)?;
        let index_expr = self.compile_expression(insert_index)?;

        let index_value = match index_expr {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow!("Vector index must be an integer")),
        };

        // Get the vector variable (which stores a pointer to heap-allocated vector)
        let (vector_type, vector_var_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;

        // Load the current vector pointer (points to [size][data...])
        let original_vector_ptr = self
            .builder
            .build_load(
                self.context.ptr_type(AddressSpace::default()),
                *vector_var_ptr,
                &format!("{}_old_vector_ptr", vector_name),
            )?
            .into_pointer_value();

        // Get current size from the header
        let original_size = self
            .builder
            .build_load(
                self.context.i64_type(),
                original_vector_ptr,
                &format!("{}_current_size", vector_name),
            )?
            .into_int_value();

        // New size = current + 1
        let new_size = self.builder.build_int_add(
            original_size,
            self.context.i64_type().const_int(1, false),
            "new_size",
        )?;

        // Extract element type from stored struct type info
        let original_element_type = vector_type
            .into_struct_type()
            .get_field_type_at_index(0)
            .ok_or_else(|| anyhow!("Invalid vector struct type for '{}'", vector_name))?;

        // Calculate element size in bytes for original type
        let element_size_bytes = (match original_element_type {
            BasicTypeEnum::FloatType(ft) => self.get_float_bit_width(ft)?,
            BasicTypeEnum::IntType(it) => it.get_bit_width(),
            _ => 64,
        } + 7)
            / 8;

        // Calculate total size: 8 bytes for header + (new_size * element_size)
        let data_size = self.builder.build_int_mul(
            self.context
                .i64_type()
                .const_int(element_size_bytes as u64, false),
            new_size,
            "data_size",
        )?;

        let total_size = self.builder.build_int_add(
            data_size,
            self.context.i64_type().const_int(8, false), // header size
            "total_size",
        )?;

        // Setup malloc retry blocks
        let current_fn = self
            .current_function
            .ok_or_else(|| anyhow!("No active function"))?;
        let malloc_try_block = self.context.append_basic_block(current_fn, "malloc_try");
        let malloc_retry_block = self.context.append_basic_block(current_fn, "malloc_retry");
        let malloc_fail_block = self.context.append_basic_block(current_fn, "malloc_fail");
        let malloc_success_original_block = self
            .context
            .append_basic_block(current_fn, "malloc_success_original");
        let malloc_success_truncated_block = self
            .context
            .append_basic_block(current_fn, "malloc_success_truncated");
        let truncate_retry_block = self
            .context
            .append_basic_block(current_fn, "truncate_retry");
        let copy_end_block = self.context.append_basic_block(current_fn, "copy_end");

        // Jump to the try block to start the malloc sequence
        self.builder.build_unconditional_branch(malloc_try_block)?;

        // === MALLOC TRY BLOCK ===
        self.builder.position_at_end(malloc_try_block);

        // Allocate new vector with header
        let malloc_fn = self
            .module
            .get_function("custom_malloc")
            .ok_or_else(|| anyhow!("'custom_malloc' function not declared in module"))?;

        let new_vector_ptr = self
            .builder
            .build_call(malloc_fn, &[total_size.into()], "malloc_new_vector")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        let is_null = self
            .builder
            .build_is_null(new_vector_ptr, "is_null_first")?;
        self.builder.build_conditional_branch(
            is_null,
            malloc_retry_block,
            malloc_success_original_block,
        )?;

        // === MALLOC RETRY BLOCK ===
        self.builder.position_at_end(malloc_retry_block);

        // print malloc fail message
        let printf_fn = self
            .module
            .get_function("printf")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;

        let format_str = self.builder.build_global_string_ptr(
            "Malloc failed at %zu elements, total size: %zu\n",
            "malloc_fail_format",
        )?;

        _ = self.builder.build_call(
            printf_fn,
            &[
                format_str.as_pointer_value().into(),
                new_size.into(),
                total_size.into(),
            ],
            "printf_malloc_fail",
        );

        // Call truncate function to free up memory
        let truncate_fn = self
            .module
            .get_function("truncate_vector_data")
            .ok_or_else(|| anyhow!("'truncate_vector_data' function not declared"))?;

        let truncated_vector_ptr = self
            .builder
            .build_call(truncate_fn, &[original_vector_ptr.into()], "truncate_call")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // Check if truncation succeeded
        let truncate_null_check = self
            .builder
            .build_is_null(truncated_vector_ptr, "truncate_null_check")?;
        self.builder.build_conditional_branch(
            truncate_null_check,
            malloc_fail_block,
            truncate_retry_block,
        )?;

        // === TRUNCATE RETRY BLOCK ===
        self.builder.position_at_end(truncate_retry_block);

        // Update the variable to point to the truncated vector
        self.builder
            .build_store(*vector_var_ptr, truncated_vector_ptr)?;

        // Calculate size for truncated data (f32 elements)
        let truncated_data_size = self.builder.build_int_mul(
            self.context.i64_type().const_int(4, false), // f32 size
            new_size,
            "data_size_truncated",
        )?;

        let truncated_total_size = self.builder.build_int_add(
            truncated_data_size,
            self.context.i64_type().const_int(8, false), // header size
            "total_size_truncated",
        )?;

        // Retry malloc with smaller size
        let retry_ptr = self
            .builder
            .build_call(
                malloc_fn,
                &[truncated_total_size.into()],
                "malloc_retry_vector",
            )?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        let is_null_retry = self.builder.build_is_null(retry_ptr, "is_null_retry")?;
        self.builder.build_conditional_branch(
            is_null_retry,
            malloc_fail_block,
            malloc_success_truncated_block,
        )?;

        // === MALLOC FAIL BLOCK ===
        self.builder.position_at_end(malloc_fail_block);
        let abort_fn = self
            .module
            .get_function("abort")
            .ok_or_else(|| anyhow!("'abort' function not declared"))?;
        self.builder.build_call(abort_fn, &[], "abort_call")?;
        self.builder.build_unreachable()?;

        // === MALLOC SUCCESS ORIGINAL BLOCK (f64 elements) ===
        self.builder.position_at_end(malloc_success_original_block);

        // Store new size in header
        self.builder.build_store(new_vector_ptr, new_size)?;

        // Get current size from original vector
        let current_size_original = self
            .builder
            .build_load(
                self.context.i64_type(),
                original_vector_ptr,
                "current_size_original",
            )?
            .into_int_value();

        // Get pointers to old and new data sections
        let old_data_ptr_original = self.get_vector_data_ptr_from_vector(&original_vector_ptr)?;
        let new_data_ptr_original = self.get_vector_data_ptr_from_vector(&new_vector_ptr)?;

        // Copy elements with insertion logic for f64
        self.copy_vector_elements_with_insertion(
            current_fn,
            original_element_type,
            old_data_ptr_original,
            new_data_ptr_original,
            current_size_original,
            index_value,
            value_expr,
            "original",
        )?;

        // Free original vector and update variable
        let free_fn = self
            .module
            .get_function("free")
            .ok_or_else(|| anyhow!("'free' function not declared in module"))?;

        self.builder.build_call(
            free_fn,
            &[original_vector_ptr.into()],
            "free_original_vector",
        )?;

        self.builder.build_store(*vector_var_ptr, new_vector_ptr)?;

        self.builder.build_unconditional_branch(copy_end_block)?;

        // === MALLOC SUCCESS TRUNCATED BLOCK (f32 elements) ===
        self.builder.position_at_end(malloc_success_truncated_block);

        // Store new size in header
        self.builder.build_store(retry_ptr, new_size)?;

        // Get current size from truncated vector
        let current_size_truncated = self
            .builder
            .build_load(
                self.context.i64_type(),
                truncated_vector_ptr,
                "current_size_truncated",
            )?
            .into_int_value();

        // Get pointers to old and new data sections
        let old_data_ptr_truncated = self.get_vector_data_ptr_from_vector(&truncated_vector_ptr)?;
        let new_data_ptr_truncated = self.get_vector_data_ptr_from_vector(&retry_ptr)?;

        // Convert the insert value to f32 if needed
        let truncated_value = match value_expr {
            BasicValueEnum::FloatValue(fv) => {
                // Truncate f64 to f32
                self.builder
                    .build_float_trunc(fv, self.context.f32_type(), "truncated_insert_value")?
                    .into()
            }
            BasicValueEnum::IntValue(iv) => {
                // Convert int to f32
                self.builder
                    .build_signed_int_to_float(
                        iv,
                        self.context.f32_type(),
                        "int_to_f32_insert_value",
                    )?
                    .into()
            }
            _ => value_expr, // Hope it's already f32
        };

        // Copy elements with insertion logic for f32
        self.copy_vector_elements_with_insertion(
            current_fn,
            BasicTypeEnum::FloatType(self.context.f32_type()),
            old_data_ptr_truncated,
            new_data_ptr_truncated,
            current_size_truncated,
            index_value,
            truncated_value,
            "truncated",
        )?;

        // Free truncated vector and update variable
        let free_fn = self
            .module
            .get_function("free")
            .ok_or_else(|| anyhow!("'free' function not declared in module"))?;

        self.builder.build_call(
            free_fn,
            &[truncated_vector_ptr.into()],
            "free_truncated_vector",
        )?;

        self.builder.build_store(*vector_var_ptr, retry_ptr)?;

        self.builder.build_unconditional_branch(copy_end_block)?;

        // === COPY END BLOCK ===
        self.builder.position_at_end(copy_end_block);

        Ok(())
    }

    // Helper method to copy vector elements with insertion
    fn copy_vector_elements_with_insertion(
        &self,
        current_fn: inkwell::values::FunctionValue,
        element_type: BasicTypeEnum,
        old_data_ptr: inkwell::values::PointerValue,
        new_data_ptr: inkwell::values::PointerValue,
        current_size: inkwell::values::IntValue,
        index_value: inkwell::values::IntValue,
        insert_value: BasicValueEnum,
        suffix: &str,
    ) -> Result<()> {
        let copy_start_block = self
            .context
            .append_basic_block(current_fn, &format!("copy_start_{}", suffix));
        let copy_body_block = self
            .context
            .append_basic_block(current_fn, &format!("copy_body_{}", suffix));
        let copy_loop_end_block = self
            .context
            .append_basic_block(current_fn, &format!("copy_loop_end_{}", suffix));

        let counter_ptr = self
            .builder
            .build_alloca(self.context.i64_type(), &format!("copy_counter_{}", suffix))?;
        self.builder
            .build_store(counter_ptr, self.context.i64_type().const_int(0, false))?;

        self.builder.build_unconditional_branch(copy_start_block)?;
        self.builder.position_at_end(copy_start_block);

        // Loop condition: while counter < current_size
        let counter_val = self
            .builder
            .build_load(
                self.context.i64_type(),
                counter_ptr,
                &format!("i_{}", suffix),
            )?
            .into_int_value();
        let continue_copy = self.builder.build_int_compare(
            inkwell::IntPredicate::ULT,
            counter_val,
            current_size,
            &format!("continue_copy_{}", suffix),
        )?;

        self.builder.build_conditional_branch(
            continue_copy,
            copy_body_block,
            copy_loop_end_block,
        )?;

        self.builder.position_at_end(copy_body_block);

        // Determine if we need to shift this element
        let should_shift = self.builder.build_int_compare(
            inkwell::IntPredicate::UGE,
            counter_val,
            index_value,
            &format!("should_shift_{}", suffix),
        )?;

        let shift_then_block = self
            .context
            .append_basic_block(current_fn, &format!("shift_then_{}", suffix));
        let shift_else_block = self
            .context
            .append_basic_block(current_fn, &format!("shift_else_{}", suffix));
        let shift_cont_block = self
            .context
            .append_basic_block(current_fn, &format!("shift_cont_{}", suffix));

        self.builder
            .build_conditional_branch(should_shift, shift_then_block, shift_else_block)?;

        // Elements at or after insertion point: shift forward by 1
        self.builder.position_at_end(shift_then_block);
        let src_ptr = unsafe {
            self.builder.build_gep(
                element_type,
                old_data_ptr,
                &[counter_val],
                &format!("shift_src_{}", suffix),
            )?
        };
        let val =
            self.builder
                .build_load(element_type, src_ptr, &format!("shift_val_{}", suffix))?;

        let dest_idx = self.builder.build_int_add(
            counter_val,
            self.context.i64_type().const_int(1, false),
            &format!("dest_idx_{}", suffix),
        )?;
        let dest_ptr = unsafe {
            self.builder.build_gep(
                element_type,
                new_data_ptr,
                &[dest_idx],
                &format!("shift_dest_{}", suffix),
            )?
        };
        self.builder.build_store(dest_ptr, val)?;
        self.builder.build_unconditional_branch(shift_cont_block)?;

        // Elements before insertion point: copy directly
        self.builder.position_at_end(shift_else_block);
        let src_ptr = unsafe {
            self.builder.build_gep(
                element_type,
                old_data_ptr,
                &[counter_val],
                &format!("copy_src_{}", suffix),
            )?
        };
        let val =
            self.builder
                .build_load(element_type, src_ptr, &format!("copy_val_{}", suffix))?;

        let dest_ptr = unsafe {
            self.builder.build_gep(
                element_type,
                new_data_ptr,
                &[counter_val],
                &format!("copy_dest_{}", suffix),
            )?
        };
        self.builder.build_store(dest_ptr, val)?;
        self.builder.build_unconditional_branch(shift_cont_block)?;

        // Continue loop
        self.builder.position_at_end(shift_cont_block);
        let next_counter = self.builder.build_int_add(
            counter_val,
            self.context.i64_type().const_int(1, false),
            &format!("next_counter_{}", suffix),
        )?;
        self.builder.build_store(counter_ptr, next_counter)?;
        self.builder.build_unconditional_branch(copy_start_block)?;

        // After loop: insert the new value
        self.builder.position_at_end(copy_loop_end_block);
        let insert_ptr = unsafe {
            self.builder.build_gep(
                element_type,
                new_data_ptr,
                &[index_value],
                &format!("insert_ptr_{}", suffix),
            )?
        };
        self.builder.build_store(insert_ptr, insert_value)?;

        // Don't branch to copy_end_block here - let the caller handle it
        Ok(())
    }
}
