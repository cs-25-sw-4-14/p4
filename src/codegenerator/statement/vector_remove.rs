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
        let vector_index = &vector_remove.index;
        let index_expr = self.compile_expression(vector_index)?;

        let index_value = match index_expr {
            BasicValueEnum::IntValue(val) => val,
            _ => {
                return Err(anyhow!("Vector index must be an integer"));
            }
        };

        let (vector_type, vector_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;

        let vector_value = self
            .builder
            .build_load(*vector_type, *vector_ptr, vector_name)?;

        let vector_value = match vector_value {
            BasicValueEnum::VectorValue(val) => val,
            _ => {
                return Err(anyhow!("Variable '{}' is not a vector", vector_name));
            }
        };

        let new_vector_type = &vector_value.get_type();
        let first_element_type = new_vector_type.get_element_type();

        if new_vector_type.get_size() < 2 {
            return Err(anyhow!("cannot remove element from vector with size < 2"));
        }

        let nr_elements = new_vector_type.get_size() - (1 as u32);
        let new_vector_type = match first_element_type {
            BasicTypeEnum::FloatType(ft) => Ok(ft.vec_type(nr_elements)),
            BasicTypeEnum::IntType(it) => Ok(it.vec_type(nr_elements)),
            BasicTypeEnum::PointerType(pt) => Ok(pt.vec_type(nr_elements)),
            BasicTypeEnum::StructType(_) => Ok(self
                .context
                .ptr_type(AddressSpace::default())
                .vec_type(nr_elements)),
            _ => Err(anyhow!("Unsupported vector element type")),
        }?;

        let mut new_vector_value = new_vector_type.get_undef();

        let mut count = 0;
        for i in 0..nr_elements + 1 {
            if u64::from(i) == index_value.get_zero_extended_constant().unwrap() {
                continue;
            }
            let src_idx = self.context.i32_type().const_int(i as u64, false);
            let dst_idx = self.context.i32_type().const_int(count as u64, false);

            count += 1;
            let value = self.builder.build_extract_element(
                vector_value,
                src_idx,
                &format!("vector_extract_{}", i),
            )?;

            new_vector_value = self.builder.build_insert_element(
                new_vector_value,
                value,
                dst_idx,
                &format!("vector_insert_{}", i),
            )?;
        }

        self.builder.build_store(*vector_ptr, new_vector_value)?;

        self.variables.insert(
            vector_name.clone(),
            (
                inkwell::types::BasicTypeEnum::VectorType(new_vector_type),
                *vector_ptr,
            ),
        );

        Ok(())
    }
}
