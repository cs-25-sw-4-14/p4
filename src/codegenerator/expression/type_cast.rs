use super::CodeGenerator;
use crate::ast::Literal;
use crate::ast::{Expression, Type, TypeCast};
use anyhow::{Result, anyhow};
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    /// Determines the bit width of a given LLVM floating-point type
    /// This is a helper function to compare float types and determine casting operations
    /// 
    /// # Arguments
    /// * `float_type` - The LLVM FloatType to analyze
    /// 
    /// # Returns
    /// * `Ok(u32)` - The bit width (16, 32, or 64)
    /// * `Err` - If the float type is not recognized
    pub fn get_float_bit_width(&self, float_type: inkwell::types::FloatType<'ctx>) -> Result<u32> {
        if float_type.eq(&self.context.f16_type()) {
            Ok(16)  // Half precision (f16)
        } else if float_type.eq(&self.context.f32_type()) {
            Ok(32)  // Single precision (f32)
        } else if float_type.eq(&self.context.f64_type()) {
            Ok(64)  // Double precision (f64)
        } else {
            Err(anyhow!("Unknown floating-point type"))
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> {
    /// Compiles a type cast expression into LLVM IR
    /// This function handles various type conversions including:
    /// - Integer to integer (with sign/zero extension or truncation)
    /// - Float to float (with extension or truncation)
    /// - Integer to float (signed/unsigned conversion)
    /// - Float to integer (signed/unsigned conversion)
    /// - String to char array and vice versa
    /// 
    /// # Arguments
    /// * `type_cast` - The TypeCast AST node containing the expression and target type
    /// 
    /// # Returns
    /// * `Ok(BasicValueEnum)` - The LLVM value representing the casted result
    /// * `Err` - If the type cast is unsupported or compilation fails
    pub fn compile_type_cast(&mut self, type_cast: &TypeCast) -> Result<BasicValueEnum<'ctx>> {
        // First, compile the source expression to get its LLVM value
        let expr_value = self.compile_expression(&type_cast.expression)?;
        
        // Convert the target type from AST representation to LLVM type
        let target_type = self.get_llvm_type(&type_cast.target_type)?;

        // Pattern match on the combination of source value type and target type
        // This determines what kind of cast operation to perform
        let casted_value = match (expr_value, target_type) {
            // INTEGER TO INTEGER CASTING
            // Handles conversions between different integer types (i8, i16, i32, i64, u8, u16, u32, u64)
            (BasicValueEnum::IntValue(value), BasicTypeEnum::IntType(int_type)) => {
                // Get the bit widths of source and target types to determine the cast operation
                let src_bits = value.get_type().get_bit_width();
                let tgt_bits = int_type.get_bit_width();
                
                // Determine if the target type is signed based on the AST type
                // Signed types: I8, I16, I32, I64
                // Unsigned types: U8, U16, U32, U64
                let is_signed = matches!(
                    type_cast.target_type,
                    Type::I8 | Type::I16 | Type::I32 | Type::I64
                );

                // Choose the appropriate cast operation based on bit width comparison
                match src_bits.cmp(&tgt_bits) {
                    // Source is smaller than target - need to extend
                    std::cmp::Ordering::Less if is_signed => {
                        // Sign extend: preserve the sign bit when extending
                        // e.g., i8(-1) -> i32(-1), not i32(255)
                        self.builder
                            .build_int_s_extend(value, int_type, "sext_tmp")?
                            .into()
                    },
                    std::cmp::Ordering::Less => {
                        // Zero extend: fill with zeros when extending unsigned values
                        // e.g., u8(255) -> u32(255)
                        self.builder
                            .build_int_z_extend(value, int_type, "zext_tmp")?
                            .into()
                    },
                    // Source is larger than target - need to truncate
                    std::cmp::Ordering::Greater => {
                        // Truncate: drop the higher-order bits
                        // e.g., i32(0x12345678) -> i16(0x5678)
                        self.builder
                            .build_int_truncate(value, int_type, "trunc_tmp")?
                            .into()
                    },
                    // Same size - no conversion needed, just return as-is
                    std::cmp::Ordering::Equal => value.into(),
                }
            }

            // FLOAT TO FLOAT CASTING  
            // Handles conversions between different floating-point types (f16, f32, f64)
            (BasicValueEnum::FloatValue(value), BasicTypeEnum::FloatType(float_type)) => {
                // Get bit widths of source and target float types
                let src_bits = self.get_float_bit_width(value.get_type()).unwrap();
                let tgt_bits = self.get_float_bit_width(float_type).unwrap();

                match src_bits.cmp(&tgt_bits) {
                    // Extending precision: f32 -> f64 or f16 -> f32/f64
                    std::cmp::Ordering::Less => {
                        self.builder
                            .build_float_ext(value, float_type, "fext_tmp")?
                            .into()
                    },
                    // Reducing precision: f64 -> f32 or f64/f32 -> f16
                    // May lose precision and cause rounding
                    std::cmp::Ordering::Greater => {
                        self.builder
                            .build_float_trunc(value, float_type, "ftrunc_tmp")?
                            .into()
                    },
                    // Same precision - no conversion needed
                    std::cmp::Ordering::Equal => value.into(),
                }
            }

            // INTEGER TO FLOAT CASTING
            // Converts integer values to floating-point representation
            (BasicValueEnum::IntValue(value), BasicTypeEnum::FloatType(float_type)) => {
                // Determine if the source integer is signed by checking the original expression
                // This is a simplified check - in practice, you might want to track signedness
                // more systematically through the type system
                let is_signed = matches!(
                    type_cast.expression.as_ref(),
                    Expression::Literal(Literal::Int(_))
                );
                
                if is_signed {
                    // Signed integer to float conversion
                    // Preserves the sign: i32(-42) -> f32(-42.0)
                    self.builder
                        .build_signed_int_to_float(value, float_type, "sitofp_tmp")?
                        .into()
                } else {
                    // Unsigned integer to float conversion  
                    // Treats the bit pattern as unsigned: u32(4294967254) -> f32(4294967254.0)
                    self.builder
                        .build_unsigned_int_to_float(value, float_type, "uitofp_tmp")?
                        .into()
                }
            }

            // FLOAT TO INTEGER CASTING
            // Converts floating-point values to integer representation
            // Note: This truncates towards zero (not rounding)
            (BasicValueEnum::FloatValue(value), BasicTypeEnum::IntType(int_type)) => {
                // Determine if the target integer type is signed
                let is_signed = matches!(
                    type_cast.target_type,
                    Type::I8 | Type::I16 | Type::I32 | Type::I64
                );
                
                if is_signed {
                    // Float to signed integer conversion
                    // f32(42.7) -> i32(42), f32(-42.7) -> i32(-42)
                    self.builder
                        .build_float_to_signed_int(value, int_type, "fptosi_tmp")?
                        .into()
                } else {
                    // Float to unsigned integer conversion
                    // f32(42.7) -> u32(42), negative values may wrap around
                    self.builder
                        .build_float_to_unsigned_int(value, int_type, "fptoui_tmp")?
                        .into()
                }
            }

            // STRING TO CHAR ARRAY CASTING
            // Converts a string pointer to a character array
            // This is typically used when interfacing with C-style strings
            (BasicValueEnum::PointerValue(ptr), BasicTypeEnum::ArrayType(array_type)) => {
                // Verify that the array element type is i8 (char)
                if let BasicTypeEnum::IntType(int_type) = array_type.get_element_type() {
                    if int_type.get_bit_width() == 8 {
                        // The pointer already points to the char data, so just return it
                        ptr.into()
                    } else {
                        return Err(anyhow::anyhow!("Cannot cast to non-char array type"));
                    }
                } else {
                    return Err(anyhow::anyhow!("Cannot cast to non-array type"));
                }
            }
            
            // CHAR ARRAY TO STRING CASTING
            // Converts a character array (represented as a struct) back to a string pointer
            // This handles the reverse of the above operation
            (BasicValueEnum::StructValue(struct_value), BasicTypeEnum::PointerType(_)) => {
                // Assume the struct contains string data and can be treated as a pointer
                struct_value.into()
            }

            // UNSUPPORTED CASTING
            // If we reach this point, the combination of source and target types
            // is not supported by this implementation
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported type cast: {:?} to {:?}",
                    expr_value,
                    target_type
                ));
            }
        };

        // Return the successfully casted value
        Ok(casted_value)
    }
}