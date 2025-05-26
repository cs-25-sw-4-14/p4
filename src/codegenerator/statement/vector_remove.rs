use crate::ast::VectorRemove;
use crate::codegenerator::BasicValueEnum;
use crate::codegenerator::CodeGenerator;
use anyhow::Result;
use anyhow::anyhow;
use inkwell::AddressSpace;
use inkwell::types::BasicTypeEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_remove(&mut self, vector_remove: &VectorRemove) -> Result<()> {
        let vector_name = &vector_remove.vector_name;
        let remove_index = &vector_remove.index;
        let index_expr = self.compile_expression(remove_index)?;

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
        let old_vector_ptr = self
            .builder
            .build_load(
                self.context.ptr_type(AddressSpace::default()),
                *vector_var_ptr,
                &format!("{}_old_vector_ptr", vector_name),
            )?
            .into_pointer_value();

        // Get current size from the header
        let current_size = self
            .builder
            .build_load(
                self.context.i64_type(),
                old_vector_ptr,
                &format!("{}_current_size", vector_name),
            )?
            .into_int_value();

        // New size = current - 1
        let new_size = self.builder.build_int_sub(
            current_size,
            self.context.i64_type().const_int(1, false),
            "new_size",
        )?;

        // Extract element type from stored struct type info
        let element_type = vector_type
            .into_struct_type()
            .get_field_type_at_index(0)
            .ok_or_else(|| anyhow!("Invalid vector struct type for '{}'", vector_name))?;

        // Calculate element size in bytes
        let element_size_bytes = (match element_type {
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

        // Allocate new vector with header
        let malloc_fn = self
            .module
            .get_function("malloc")
            .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

        let new_vector_ptr = self
            .builder
            .build_call(malloc_fn, &[total_size.into()], "malloc_new_vector")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        // Store new size in header
        self.builder.build_store(new_vector_ptr, new_size)?;

        // Get pointers to old and new data sections
        let old_data_ptr = self.get_vector_data_ptr_from_vector(&old_vector_ptr)?;
        let new_data_ptr = self.get_vector_data_ptr_from_vector(&new_vector_ptr)?;

        // Copy elements with removal logic
        let current_fn = match self.current_function {
            Some(function) => function,
            None => return Err(anyhow!("No active function context for vector remove")),
        };

        let copy_start_block = self.context.append_basic_block(current_fn, "copy_start");
        let copy_body_block = self.context.append_basic_block(current_fn, "copy_body");
        let copy_end_block = self.context.append_basic_block(current_fn, "copy_end");

        let counter_ptr = self
            .builder
            .build_alloca(self.context.i64_type(), "copy_counter")?;
        self.builder
            .build_store(counter_ptr, self.context.i64_type().const_int(0, false))?;

        self.builder.build_unconditional_branch(copy_start_block)?;
        self.builder.position_at_end(copy_start_block);

        // Loop condition: while counter < current_size
        let counter_val = self
            .builder
            .build_load(self.context.i64_type(), counter_ptr, "i")?
            .into_int_value();
        let continue_copy = self.builder.build_int_compare(
            inkwell::IntPredicate::ULT,
            counter_val,
            current_size,
            "continue_copy",
        )?;

        self.builder
            .build_conditional_branch(continue_copy, copy_body_block, copy_end_block)?;
        self.builder.position_at_end(copy_body_block);

        // Check if this is the element to remove
        let is_remove_index = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            counter_val,
            index_value,
            "is_remove_index",
        )?;

        let skip_block = self.context.append_basic_block(current_fn, "skip_remove");
        let copy_element_block = self.context.append_basic_block(current_fn, "copy_element");
        let copy_cont_block = self.context.append_basic_block(current_fn, "copy_cont");

        self.builder
            .build_conditional_branch(is_remove_index, skip_block, copy_element_block)?;

        // Skip the element to be removed
        self.builder.position_at_end(skip_block);
        self.builder.build_unconditional_branch(copy_cont_block)?;

        // Copy element with proper destination index adjustment
        self.builder.position_at_end(copy_element_block);
        let src_ptr = unsafe {
            self.builder
                .build_gep(element_type, old_data_ptr, &[counter_val], "copy_src")?
        };
        let val = self.builder.build_load(element_type, src_ptr, "copy_val")?;

        // Determine destination index: if counter > remove_index, shift left by 1
        let should_shift = self.builder.build_int_compare(
            inkwell::IntPredicate::UGT,
            counter_val,
            index_value,
            "should_shift",
        )?;

        let dest_idx = self
            .builder
            .build_select(
                should_shift,
                self.builder.build_int_sub(
                    counter_val,
                    self.context.i64_type().const_int(1, false),
                    "shifted_idx",
                )?,
                counter_val,
                "dest_idx",
            )?
            .into_int_value();

        let dest_ptr = unsafe {
            self.builder
                .build_gep(element_type, new_data_ptr, &[dest_idx], "copy_dest")?
        };
        self.builder.build_store(dest_ptr, val)?;
        self.builder.build_unconditional_branch(copy_cont_block)?;

        // Continue loop
        self.builder.position_at_end(copy_cont_block);
        let next_counter = self.builder.build_int_add(
            counter_val,
            self.context.i64_type().const_int(1, false),
            "next_counter",
        )?;
        self.builder.build_store(counter_ptr, next_counter)?;
        self.builder.build_unconditional_branch(copy_start_block)?;

        // After loop: cleanup
        self.builder.position_at_end(copy_end_block);

        // Free the old vector
        let free_fn = self
            .module
            .get_function("free")
            .ok_or_else(|| anyhow!("'free' function not declared in module"))?;

        self.builder
            .build_call(free_fn, &[old_vector_ptr.into()], "free_old_vector")?;

        // Update the variable to point to the new vector
        self.builder.build_store(*vector_var_ptr, new_vector_ptr)?;

        Ok(())
    }
}
