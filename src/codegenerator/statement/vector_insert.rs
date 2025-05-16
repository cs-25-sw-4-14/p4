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
        let vector_index = &vector_insert.index;
        let vector_value = &vector_insert.value;
        let value_expr = self.compile_expression(vector_value)?;
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

        let nr_elements = new_vector_type.get_size() + (1 as u32);
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

        let insert_idx = index_value
            .get_zero_extended_constant()
            .ok_or_else(|| anyhow!("Vector insert index must be a constant"))? as u32;

        let mut new_vector_value = new_vector_type.get_undef();
        let mut dst_pos = 0;

        for i in 0..new_vector_type.get_size() {
            let dst_idx = self.context.i32_type().const_int(dst_pos as u64, false);

            let inserted_value = if i == insert_idx {
                // insert the new value at this position
                value_expr
            } else {
                // extract from the old vector, shifting elements >= insert_idx right
                let src_pos = if i > insert_idx { i - 1 } else { i };
                let src_idx = self.context.i32_type().const_int(src_pos as u64, false);
                self.builder.build_extract_element(
                    vector_value,
                    src_idx,
                    &format!("vector_extract_{}", i),
                )?
            };

            new_vector_value = self.builder.build_insert_element(
                new_vector_value,
                inserted_value,
                dst_idx,
                &format!("vector_insert_{}", i),
            )?;

            dst_pos += 1;
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
