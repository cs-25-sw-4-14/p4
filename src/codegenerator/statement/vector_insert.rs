use crate::ast::VectorInsert;
use crate::codegenerator::BasicValueEnum;
use crate::codegenerator::CodeGenerator;
use anyhow::Result;
use anyhow::anyhow;
use inkwell::AddressSpace;
use inkwell::types::BasicTypeEnum;

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a vector insert operation, inserting a new value into the vector at a given index.
    /// This involves reallocating memory to fit the new size, copying over existing elements,
    /// shifting them if needed, inserting the new value, and finally freeing the old vector.
    pub fn compile_vector_insert(&mut self, vector_insert: &VectorInsert) -> Result<()> {
        // Extract parts of the VectorInsert AST node
        let vector_name = &vector_insert.vector_name;
        let insert_index = &vector_insert.index;
        let insert_value = &vector_insert.value;

        // Compile the value to be inserted (must be a valid expression)
        let value_expr = self.compile_expression(insert_value)?;

        // Compile the index expression (where to insert the new value)
        let index_expr = self.compile_expression(insert_index)?;
        let index_value = match index_expr {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow!("Vector index must be an integer")),
        };

        // Look up the variable (which holds a pointer to the heap-allocated vector)
        let (vector_type, vector_var_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;

        // Load the current pointer to the vector ([size][data...])
        let old_vector_ptr = self
            .builder
            .build_load(
                self.context.ptr_type(AddressSpace::default()),
                *vector_var_ptr,
                &format!("{}_old_vector_ptr", vector_name),
            )?
            .into_pointer_value();

        // Read the current vector size (stored in the first 8 bytes of the vector memory)
        let current_size = self
            .builder
            .build_load(
                self.context.i64_type(),
                old_vector_ptr,
                &format!("{}_current_size", vector_name),
            )?
            .into_int_value();

        // Compute new size = current size + 1
        let new_size = self.builder.build_int_add(
            current_size,
            self.context.i64_type().const_int(1, false),
            "new_size",
        )?;

        // Determine the type of elements stored in the vector
        let element_type = vector_type
            .into_struct_type()
            .get_field_type_at_index(0)
            .ok_or_else(|| anyhow!("Invalid vector struct type for '{}'", vector_name))?;

        // Determine the size of each element in bytes
        let element_size_bytes = (match element_type {
            BasicTypeEnum::FloatType(ft) => self.get_float_bit_width(ft)?,
            BasicTypeEnum::IntType(it) => it.get_bit_width(),
            _ => 64, // fallback for unknown types
        } + 7) / 8;

        // Compute space required for new data section
        let data_size = self.builder.build_int_mul(
            self.context
                .i64_type()
                .const_int(element_size_bytes as u64, false),
            new_size,
            "data_size",
        )?;

        // Total memory needed = 8-byte header + data
        let total_size = self.builder.build_int_add(
            data_size,
            self.context.i64_type().const_int(8, false),
            "total_size",
        )?;

        // Allocate new memory block
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

        // Store the new size in the header of the new vector
        self.builder.build_store(new_vector_ptr, new_size)?;

        // Get pointers to the data section (skipping 8-byte size header)
        let old_data_ptr = self.get_vector_data_ptr_from_vector(&old_vector_ptr)?;
        let new_data_ptr = self.get_vector_data_ptr_from_vector(&new_vector_ptr)?;

        // Control flow setup: allocate blocks for copying
        let current_fn = self.current_function.ok_or_else(|| {
            anyhow!("No active function context for vector insert")
        })?;

        let copy_start_block = self.context.append_basic_block(current_fn, "copy_start");
        let copy_body_block = self.context.append_basic_block(current_fn, "copy_body");
        let copy_end_block = self.context.append_basic_block(current_fn, "copy_end");

        // Allocate and initialize a loop counter variable
        let counter_ptr = self
            .builder
            .build_alloca(self.context.i64_type(), "copy_counter")?;
        self.builder
            .build_store(counter_ptr, self.context.i64_type().const_int(0, false))?;

        // Begin loop to copy/shift elements
        self.builder.build_unconditional_branch(copy_start_block)?;
        self.builder.position_at_end(copy_start_block);

        let counter_val = self
            .builder
            .build_load(self.context.i64_type(), counter_ptr, "i")?
            .into_int_value();

        // Loop condition: i < current_size
        let continue_copy = self.builder.build_int_compare(
            inkwell::IntPredicate::ULT,
            counter_val,
            current_size,
            "continue_copy",
        )?;

        self.builder
            .build_conditional_branch(continue_copy, copy_body_block, copy_end_block)?;
        self.builder.position_at_end(copy_body_block);

        // Decide if current element should be shifted (i >= insert index)
        let should_shift = self.builder.build_int_compare(
            inkwell::IntPredicate::UGE,
            counter_val,
            index_value,
            "should_shift",
        )?;

        // Create control blocks for conditional copying logic
        let shift_then_block = self.context.append_basic_block(current_fn, "shift_then");
        let shift_else_block = self.context.append_basic_block(current_fn, "shift_else");
        let shift_cont_block = self.context.append_basic_block(current_fn, "shift_cont");

        self.builder
            .build_conditional_branch(should_shift, shift_then_block, shift_else_block)?;

        // Case 1: shift elements (for indices >= insert_index)
        self.builder.position_at_end(shift_then_block);
        let src_ptr = unsafe {
            self.builder
                .build_gep(element_type, old_data_ptr, &[counter_val], "shift_src")?
        };
        let val = self
            .builder
            .build_load(element_type, src_ptr, "shift_val")?;

        let dest_idx = self.builder.build_int_add(
            counter_val,
            self.context.i64_type().const_int(1, false),
            "dest_idx",
        )?;
        let dest_ptr = unsafe {
            self.builder
                .build_gep(element_type, new_data_ptr, &[dest_idx], "shift_dest")?
        };
        self.builder.build_store(dest_ptr, val)?;
        self.builder.build_unconditional_branch(shift_cont_block)?;

        // Case 2: copy elements directly for indices < insert_index
        self.builder.position_at_end(shift_else_block);
        let src_ptr = unsafe {
            self.builder
                .build_gep(element_type, old_data_ptr, &[counter_val], "copy_src")?
        };
        let val = self.builder.build_load(element_type, src_ptr, "copy_val")?;

        let dest_ptr = unsafe {
            self.builder
                .build_gep(element_type, new_data_ptr, &[counter_val], "copy_dest")?
        };
        self.builder.build_store(dest_ptr, val)?;
        self.builder.build_unconditional_branch(shift_cont_block)?;

        // Merge point: increment loop counter
        self.builder.position_at_end(shift_cont_block);
        let next_counter = self.builder.build_int_add(
            counter_val,
            self.context.i64_type().const_int(1, false),
            "next_counter",
        )?;
        self.builder.build_store(counter_ptr, next_counter)?;
        self.builder.build_unconditional_branch(copy_start_block)?;

        // After loop finishes, insert the new value at the specified index
        self.builder.position_at_end(copy_end_block);
        let insert_ptr = unsafe {
            self.builder
                .build_gep(element_type, new_data_ptr, &[index_value], "insert_ptr")?
        };
        self.builder.build_store(insert_ptr, value_expr)?;

        // Deallocate old memory (no memory leak!)
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
