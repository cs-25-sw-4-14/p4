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

        let (vector_type, vector_struct_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;

        let data_ptr_ptr = self.builder.build_struct_gep(
            *vector_type,
            *vector_struct_ptr,
            0,
            &format!("{}_data_ptr", vector_name),
        )?;

        let old_heap_ptr = self
            .builder
            .build_load(
                self.context.ptr_type(AddressSpace::default()),
                data_ptr_ptr,
                &format!("{}_old_heap_ptr", vector_name),
            )?
            .into_pointer_value();

        let size_ptr = self.builder.build_struct_gep(
            *vector_type,
            *vector_struct_ptr,
            1,
            &format!("{}_size_ptr", vector_name),
        )?;

        let current_size = self
            .builder
            .build_load(
                self.context.i64_type(),
                size_ptr,
                &format!("{}_current_size", vector_name),
            )?
            .into_int_value();

        // new size = +1
        let new_size = self.builder.build_int_add(
            current_size,
            self.context.i64_type().const_int(1, false),
            "new_size",
        )?;

        let element_type = vector_type
            .into_struct_type()
            .get_field_type_at_index(1)
            .unwrap();

        // Calculate the element size
        let element_size_bytes = (match element_type {
            BasicTypeEnum::FloatType(ft) => self.get_float_bit_width(ft)?,
            BasicTypeEnum::IntType(it) => it.get_bit_width(),
            _ => 64,
        } + 7)
            / 8;

        // multiply new_size by element_size
        let total_size = self.builder.build_int_mul(
            self.context
                .i64_type()
                .const_int(element_size_bytes as u64, false),
            new_size,
            "total_size",
        )?;

        let malloc_fn = self
            .module
            .get_function("malloc")
            .ok_or_else(|| anyhow!("'malloc' function not declared in module"))?;

        let new_heap_ptr = self
            .builder
            .build_call(malloc_fn, &[total_size.into()], "malloc")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        let current_fn = match self.current_function {
            Some(function) => function,
            None => return Err(anyhow!("No active function context for vector insert")),
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

        // loop condition
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

        // handle shifting of elements based on the insertion index
        let should_shift = self.builder.build_int_compare(
            inkwell::IntPredicate::UGE,
            counter_val,
            index_value,
            "should_shift",
        )?;

        let shift_then_block = self.context.append_basic_block(current_fn, "shift_then");
        let shift_else_block = self.context.append_basic_block(current_fn, "shift_else");
        let shift_cont_block = self.context.append_basic_block(current_fn, "shift_cont");

        self.builder
            .build_conditional_branch(should_shift, shift_then_block, shift_else_block)?;

        // For elements at or after the insertion point, shift them one position forward
        // This might not be the best way to do this
        self.builder.position_at_end(shift_then_block);
        let src_ptr = unsafe {
            self.builder
                .build_gep(element_type, old_heap_ptr, &[counter_val], "shift_src")?
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
                .build_gep(element_type, new_heap_ptr, &[dest_idx], "shift_dest")?
        };
        self.builder.build_store(dest_ptr, val)?;
        self.builder.build_unconditional_branch(shift_cont_block)?;

        // for elements before the insertion point
        self.builder.position_at_end(shift_else_block);
        let src_ptr = unsafe {
            self.builder
                .build_gep(element_type, old_heap_ptr, &[counter_val], "copy_src")?
        };
        let val = self.builder.build_load(element_type, src_ptr, "copy_val")?;

        let dest_ptr = unsafe {
            self.builder
                .build_gep(element_type, new_heap_ptr, &[counter_val], "copy_dest")?
        };
        self.builder.build_store(dest_ptr, val)?;
        self.builder.build_unconditional_branch(shift_cont_block)?;

        self.builder.position_at_end(shift_cont_block);
        let next_counter = self.builder.build_int_add(
            counter_val,
            self.context.i64_type().const_int(1, false),
            "next_counter",
        )?;
        self.builder.build_store(counter_ptr, next_counter)?;
        self.builder.build_unconditional_branch(copy_start_block)?;

        self.builder.position_at_end(copy_end_block);
        let insert_ptr = unsafe {
            self.builder
                .build_gep(element_type, new_heap_ptr, &[index_value], "insert_ptr")?
        };
        self.builder.build_store(insert_ptr, value_expr)?;

        // free the old heap pointer
        let free_fn = self
            .module
            .get_function("free")
            .ok_or_else(|| anyhow!("'free' function not declared in module"))?;

        self.builder
            .build_call(free_fn, &[old_heap_ptr.into()], "free_old_heap")?;

        // Update the vector data pointer
        self.builder.build_store(data_ptr_ptr, new_heap_ptr)?;

        // Update the size
        self.builder.build_store(size_ptr, new_size)?;

        // Update the size in our tracking map as well
        self.heap_sizes.insert(*vector_struct_ptr, new_size);

        Ok(())
    }
}
