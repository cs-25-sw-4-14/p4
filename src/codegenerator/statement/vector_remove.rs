
use crate::codegenerator::BasicValueEnum;
use anyhow::Result;
use anyhow::anyhow;
use crate::ast::VectorRemove;
use crate::codegenerator::CodeGenerator;

impl <'ctx> CodeGenerator<'ctx> {
    pub fn compile_vector_remove(
        &mut self,
        vector_remove: &VectorRemove,
    ) -> Result<()>
    {
        let vector_name = &vector_remove.vector_name;
        let vector_index = &vector_remove.index;
        let index_expr = self.compile_expression(vector_index)?;

        let index_value = match index_expr {
            BasicValueEnum::IntValue(val) => val,
            _ => return Err(anyhow!("Vector index must be an integer")),
        };

        let (vector_type, vector_ptr) = self
            .variables
            .get(vector_name)
            .ok_or_else(|| anyhow!("Vector variable '{}' not found", vector_name))?;
        
        let vector_value = self.builder.build_load(*vector_type, *vector_ptr, vector_name)?;
        
        let vector_value = match vector_value {
            BasicValueEnum::VectorValue(val) => val,
            _ => return Err(anyhow!("Variable '{}' is not a vector", vector_name)),
        };

        // extract the number of elements from the vector type
        let vector_type = vector_value.get_type();
        let types = vector_type.print_to_string().to_string(); // <"2 x i64">
        let trimmed_types = types.trim_matches(&['<', '>'][..]); // "2 x i64"
        let parts: Vec<&str> = trimmed_types.split('x').collect();
        let num_elements_str = parts[0].trim(); // "2"
        let num_elements: u64 = num_elements_str.parse().unwrap_or(0);

        let new_vector = 


        
        for i in 0..num_elements {
            let index = self.context.i64_type().const_int(i, false);
            let index_value_int = index_value.get_zero_extended_constant()
                .ok_or_else(|| anyhow!("Failed to get constant value from index"))?;

            if i == index_value_int {
           
                continue;
            }
            let element_value = self.builder.build_extract_element(
                vector_value,
                index,
                &format!("{}_element", vector_name)
            )?;




            println!("Element {}: {:?}", i, element_value);
        };


        println!("Number of elements: {}", num_elements);

        // 

        Ok(())











    }

}