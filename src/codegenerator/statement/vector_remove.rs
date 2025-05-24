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
        let index_expr = self.compile_expression(&vector_remove.index)?;

        let index_value = match index_expr {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow!("Vector index must be an integer")),
        };

        let (vector_type, vector_struct_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector '{}' not found", vector_name))?;

        let data_ptr_ptr =
            self.builder
                .build_struct_gep(*vector_type, *vector_struct_ptr, 0, "data_ptr_ptr")?;
        let size_ptr =
            self.builder
                .build_struct_gep(*vector_type, *vector_struct_ptr, 1, "size_ptr")?;

        let old_heap_ptr = self
            .builder
            .build_load(
                self.context.ptr_type(AddressSpace::default()),
                data_ptr_ptr,
                "old_heap_ptr",
            )?
            .into_pointer_value();

        let current_size = self
            .builder
            .build_load(self.context.i64_type(), size_ptr, "current_size")?
            .into_int_value();

        // new_size = current_size - 1
        let new_size = self.builder.build_int_sub(
            current_size,
            self.context.i64_type().const_int(1, false),
            "new_size",
        )?;

        let element_type = vector_type
            .into_struct_type()
            .get_field_type_at_index(0)
            .unwrap();

        let element_size_bytes = (match element_type {
            BasicTypeEnum::IntType(it) => it.get_bit_width(),
            BasicTypeEnum::FloatType(ft) => self.get_float_bit_width(ft)?,
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

        // malloc(new_size * element_size) in bytes
        let malloc_fn = self
            .module
            .get_function("malloc")
            .ok_or_else(|| anyhow!("Missing malloc"))?;

        let new_heap_ptr = self
            .builder
            .build_call(malloc_fn, &[total_size.into()], "new_heap")?
            .try_as_basic_value()
            .left()
            .unwrap()
            .into_pointer_value();

        let current_fn = self
            .current_function
            .ok_or_else(|| anyhow!("No current function"))?;
        let loop_cond = self.context.append_basic_block(current_fn, "loop_cond");
        let loop_body = self.context.append_basic_block(current_fn, "loop_body");
        let loop_end = self.context.append_basic_block(current_fn, "loop_end");

        let i_ptr = self.builder.build_alloca(self.context.i64_type(), "i")?;
        self.builder
            .build_store(i_ptr, self.context.i64_type().const_zero())?;
        self.builder.build_unconditional_branch(loop_cond)?;

        // loop_cond:
        self.builder.position_at_end(loop_cond);
        let i_val = self
            .builder
            .build_load(self.context.i64_type(), i_ptr, "i_val")?
            .into_int_value();

        let cond = self.builder.build_int_compare(
            inkwell::IntPredicate::ULT,
            i_val,
            current_size,
            "cond",
        )?;
        self.builder
            .build_conditional_branch(cond, loop_body, loop_end)?;

        // loop_body:
        self.builder.position_at_end(loop_body);

        // if i != index_value
        let is_not_index = self.builder.build_int_compare(
            inkwell::IntPredicate::NE,
            i_val,
            index_value,
            "is_not_index",
        )?;

        let copy_block = self.context.append_basic_block(current_fn, "copy_block");
        let skip_block = self.context.append_basic_block(current_fn, "skip_block");

        _ = self
            .builder
            .build_conditional_branch(is_not_index, copy_block, skip_block);

        // copy_block:
        self.builder.position_at_end(copy_block);

        // dest_index = if i < index { i } else { i - 1 }
        let is_before = self.builder.build_int_compare(
            inkwell::IntPredicate::ULT,
            i_val,
            index_value,
            "is_before",
        )?;
        let i_minus_one = self.builder.build_int_sub(
            i_val,
            self.context.i64_type().const_int(1, false),
            "i_minus_one",
        )?;
        let dest_index = self
            .builder
            .build_select(is_before, i_val, i_minus_one, "dest_index")?
            .into_int_value();

        let src_ptr = unsafe {
            self.builder
                .build_gep(element_type, old_heap_ptr, &[i_val], "src_ptr")?
        };
        let val = self.builder.build_load(element_type, src_ptr, "val")?;

        let dest_ptr = unsafe {
            self.builder
                .build_gep(element_type, new_heap_ptr, &[dest_index], "dest_ptr")?
        };
        self.builder.build_store(dest_ptr, val)?;
        self.builder.build_unconditional_branch(skip_block)?;

        // skip_block:
        self.builder.position_at_end(skip_block);
        let next_i = self.builder.build_int_add(
            i_val,
            self.context.i64_type().const_int(1, false),
            "next_i",
        )?;
        self.builder.build_store(i_ptr, next_i)?;
        self.builder.build_unconditional_branch(loop_cond)?;

        // loop_end:
        self.builder.position_at_end(loop_end);

        // free the old heap pointer
        let free_fn = self
            .module
            .get_function("free")
            .ok_or_else(|| anyhow!("'free' function not declared in module"))?;

        self.builder
            .build_call(free_fn, &[old_heap_ptr.into()], "free_old_heap")?;

        self.builder.build_store(data_ptr_ptr, new_heap_ptr)?;
        self.builder.build_store(size_ptr, new_size)?;

        Ok(())
    }
}
