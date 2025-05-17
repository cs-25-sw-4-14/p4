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
        println!("Vector index: {:?}", index_value);

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

        let nr_elements = new_vector_type.get_size() + 1;
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

        for i in 0..new_vector_type.get_size() {
            let idx = self.context.i64_type().const_int(i as u64, false);

            let old_elem = self.builder.build_extract_element(
                vector_value,
                idx,
                &format!("vector_extract_{}", i),
            )?;

            let should_insert = self.builder.build_int_compare(
                inkwell::IntPredicate::EQ,
                index_value,
                idx,
                &format!("cmp_idx_{}", i),
            )?;

            let new_elem = self.builder.build_select(
                should_insert,
                value_expr,
                old_elem,
                &format!("select_{}", i),
            )?;

            new_vector_value = self.builder.build_insert_element(
                new_vector_value,
                new_elem,
                idx,
                &format!("vector_insert_{}", i),
            )?;
        }

        let new_vector_ptr = self
            .builder
            .build_alloca(new_vector_type, &format!("{}_new", vector_name))?;

        self.builder.build_store(new_vector_ptr, new_vector_value)?;

        self.variables.insert(
            vector_name.clone(),
            (
                inkwell::types::BasicTypeEnum::VectorType(new_vector_type),
                new_vector_ptr,
            ),
        );

        Ok(())
    }
}
