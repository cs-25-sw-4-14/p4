use crate::{ast::Print, codegenerator::CodeGenerator};
use anyhow::{Result, anyhow};
use inkwell::values::{BasicValueEnum, BasicMetadataValueEnum};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_print_stmt(&mut self, stmt: &Print) -> Result<()> {
        let value = self.compile_expression(&stmt.expression)?;

        let printf = self
            .module
            .get_function("printf")
            .ok_or_else(|| anyhow!("'printf' function not declared in module"))?;

        let call_args: Vec<BasicMetadataValueEnum<'ctx>> = match value {
            BasicValueEnum::IntValue(int_val) => {
                match int_val.get_type().get_bit_width() {
                    1 => {
                        // Special case for boolean values
                        let fmt_str = self.builder.build_global_string_ptr("%s", "bool_fmt")?;
                        
                        // Create a conditional branch to select "true" or "false" string
                        let current_block = self.builder.get_insert_block().unwrap();
                        let function = current_block.get_parent().unwrap();
                        
                        let true_block = self.context.append_basic_block(function, "print_true");
                        let false_block = self.context.append_basic_block(function, "print_false");
                        let merge_block = self.context.append_basic_block(function, "print_merge");
                        
                        self.builder.build_conditional_branch(int_val, true_block, false_block);
                        
                        // True block
                        self.builder.position_at_end(true_block);
                        let true_str = self.builder.build_global_string_ptr("true", "true_str")?;
                        self.builder.build_unconditional_branch(merge_block);
                        
                        // False block
                        self.builder.position_at_end(false_block);
                        let false_str = self.builder.build_global_string_ptr("false", "false_str")?;
                        self.builder.build_unconditional_branch(merge_block);
                        
                        // Merge block
                        self.builder.position_at_end(merge_block);
                        
                        // Create PHI node to select the appropriate string
                        let phi_type = self.context.ptr_type(inkwell::AddressSpace::default());
                        let phi = self.builder.build_phi(phi_type, "bool_str")?;
                        
                        phi.add_incoming(&[
                            (&true_str.as_pointer_value(), true_block),
                            (&false_str.as_pointer_value(), false_block)
                        ]);
                        
                        let basic_val = phi.as_basic_value();
                        match basic_val {
                            BasicValueEnum::PointerValue(ptr) => {
                                vec![
                                    BasicMetadataValueEnum::PointerValue(fmt_str.as_pointer_value()),
                                    BasicMetadataValueEnum::PointerValue(ptr)
                                ]
                            }
                            _ => return Err(anyhow!("Expected pointer value from phi node"))
                        }
                    },
                    8 => {
                        let fmt_str = self.builder.build_global_string_ptr("%hhi", "fmt_i8")?;
                        vec![
                            BasicMetadataValueEnum::PointerValue(fmt_str.as_pointer_value()),
                            BasicMetadataValueEnum::IntValue(int_val)
                        ]
                    }
                    16 => {
                        let fmt_str = self.builder.build_global_string_ptr("%hi", "fmt_i16")?;
                        vec![
                            BasicMetadataValueEnum::PointerValue(fmt_str.as_pointer_value()),
                            BasicMetadataValueEnum::IntValue(int_val)
                        ]
                    }
                    32 => {
                        let fmt_str = self.builder.build_global_string_ptr("%d", "fmt_i32")?;
                        vec![
                            BasicMetadataValueEnum::PointerValue(fmt_str.as_pointer_value()),
                            BasicMetadataValueEnum::IntValue(int_val)
                        ]
                    }
                    64 => {
                        let fmt_str = self.builder.build_global_string_ptr("%lld", "fmt_i64")?;
                        vec![
                            BasicMetadataValueEnum::PointerValue(fmt_str.as_pointer_value()),
                            BasicMetadataValueEnum::IntValue(int_val)
                        ]
                    }
                    other => {
                        return Err(anyhow!("Unsupported integer bit width: {}", other));
                    }
                }
            }
            BasicValueEnum::FloatValue(float_val) => {
                let fmt_str = self.builder.build_global_string_ptr("%lf", "fmt")?;
                vec![
                    BasicMetadataValueEnum::PointerValue(fmt_str.as_pointer_value()),
                    BasicMetadataValueEnum::FloatValue(float_val)
                ]
            }
            BasicValueEnum::PointerValue(ptr_val) => {
                // Pointers could be string values or struct pointers
                // For now, just print them as strings
                let fmt_str = self.builder.build_global_string_ptr("%s", "fmt")?;
                vec![
                    BasicMetadataValueEnum::PointerValue(fmt_str.as_pointer_value()),
                    BasicMetadataValueEnum::PointerValue(ptr_val)
                ]
            }
            _ => return Err(anyhow!("not a valid type for print statement")),
        };

        let _ = self.builder.build_call(printf, &call_args, "printf_call");

        Ok(())
    }
}
