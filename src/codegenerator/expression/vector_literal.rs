

// impl <'ctx> CodeGenerator<'ctx> {
//     pub fn compile_vector_literal(
//         &mut self,
//         vector_literal: &Box<VectorLiteral>,
//     ) -> Result<BasicValueEnum<'ctx>>{

//         // Make sure the array is not empty
//         let first_element = vector_literal
//             .elements
//             .first()
//             .ok_or_else(|| anyhow::anyhow!("Cannot create an array from an empty literal"))?;
//         // Compile the first element and get its type
//         let first_value = self.compile_expression(first_element)?;
//         let element_type = first_value.get_type();

//         // Compile all element expressions and ensure they're of the same type
//         let mut element_values = Vec::new();
//         element_values.push(first_value);

//         for element_expr in vector_literal.elements.iter().skip(1) {
//             let value = self.compile_expression(element_expr)?;
//             // Optionally check type equality here
//             if value.get_type() != element_type {
//                 return Err(anyhow::anyhow!("Array elements must be of the same type"));
//             }
//             element_values.push(value);
//         }


//         // Create LLVM array type and allocate it
//         let vector_type = element_type.VectorType(element_values.len() as u32);
//         let vector_ptr = self.builder.build_alloca(vector_type, "vector_literal")?;

//         // Store each value into the vector
//         for (i, value) in element_values.iter().enumerate() {
//             let zero = self.context.i32_type().const_zero();
//             let index = self.context.i32_type().const_int(i as u64, false);
//             let indices = [zero, index];

//             let element_ptr = unsafe {
//                 self.builder.build_in_bounds_gep(
//                     vector_type,
//                     vector_ptr,
//                     &indices,
//                     &format!("vector_element_ptr_{}", i),
//                 )
//             }?;

//             self.builder.build_store(element_ptr, *value)?;
//         }
//         Ok(BasicValueEnum::PointerValue(vector_ptr))
//     }



// }