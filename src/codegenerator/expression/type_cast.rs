use crate::ast::{Expression, TypeCast, Type};
use super::CodeGenerator;
use anyhow::Result;
use crate::ast::Literal;
use inkwell::values::BasicValueEnum;
use inkwell::types::BasicTypeEnum;
use inkwell::types::BasicType;

impl<'ctx> CodeGenerator<'ctx> {
    // Function to get the bit width of a float type (32-bit or 64-bit)
    fn get_float_bit_width<T>(&self, float_val_or_type: T) -> Option<u32>
    where
        T: Into<BasicTypeEnum<'ctx>> + Copy,
    {
        let ty = float_val_or_type.into();
        if ty == self.context.f32_type().as_basic_type_enum() {
            Some(32) // 32-bit float
        } else if ty == self.context.f64_type().as_basic_type_enum() {
            Some(64) // 64-bit float
        } else {
            None // Unsupported float type
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> {
    // Function to compile a type cast expression
    pub fn compile_type_cast(&mut self, type_cast: &TypeCast) -> Result<BasicValueEnum<'ctx>> {
        // Compile the expression to a value
        let expr_value = self.compile_expression(&type_cast.expression)?;
        // Get the target type for the cast
        let target_type = self.get_basic_type(&type_cast.target_type);

        // Match on the types of expression and target type
        let casted_value = match (expr_value, target_type) {
            // Integer to Integer casting
            (BasicValueEnum::IntValue(value), BasicTypeEnum::IntType(int_type)) => {
                let src_bits = value.get_type().get_bit_width();
                let tgt_bits = int_type.get_bit_width();
                // Determine if the target type is signed or unsigned
                let is_signed = matches!(type_cast.target_type, Type::I8 | Type::I16 | Type::I32 | Type::I64);

                // Perform the appropriate extension or truncation based on bit widths
                if src_bits < tgt_bits {
                    if is_signed {
                        // Signed extension
                        self.builder.build_int_s_extend(value, int_type, "sext_tmp")?.into()
                    } else {
                        // Zero extension
                        self.builder.build_int_z_extend(value, int_type, "zext_tmp")?.into()
                    }
                } else if src_bits > tgt_bits {
                    // Truncation
                    self.builder.build_int_truncate(value, int_type, "trunc_tmp")?.into()
                } else {
                    // No change needed
                    value.into()
                }
            }

            // Float to Float casting
            (BasicValueEnum::FloatValue(value), BasicTypeEnum::FloatType(float_type)) => {
                let src_bits = self
                    .get_float_bit_width(value.get_type().as_basic_type_enum())
                    .ok_or_else(|| anyhow::anyhow!("Unsupported source float type"))?;
                let tgt_bits = self
                    .get_float_bit_width(float_type.as_basic_type_enum())
                    .ok_or_else(|| anyhow::anyhow!("Unsupported target float type"))?;

                // Perform the appropriate float extension or truncation based on bit widths
                if src_bits < tgt_bits {
                    self.builder.build_float_ext(value, float_type, "fext_tmp")?.into()
                } else if src_bits > tgt_bits {
                    self.builder.build_float_trunc(value, float_type, "ftrunc_tmp")?.into()
                } else {
                    value.into()
                }
            }

            // Integer to Float casting
            (BasicValueEnum::IntValue(value), BasicTypeEnum::FloatType(float_type)) => {
                // Check if the integer is signed
                let is_signed = matches!(type_cast.expression.as_ref(), Expression::Literal(Literal::Int(_)));
                if is_signed {
                    // Signed integer to float
                    self.builder.build_signed_int_to_float(value, float_type, "sitofp_tmp")?.into()
                } else {
                    // Unsigned integer to float
                    self.builder.build_unsigned_int_to_float(value, float_type, "uitofp_tmp")?.into()
                }
            }

            // Float to Integer casting
            (BasicValueEnum::FloatValue(value), BasicTypeEnum::IntType(int_type)) => {
                // Check if the target type is signed or unsigned
                let is_signed = matches!(type_cast.target_type, Type::I8 | Type::I16 | Type::I32 | Type::I64);
                if is_signed {
                    // Float to signed integer
                    self.builder.build_float_to_signed_int(value, int_type, "fptosi_tmp")?.into()
                } else {
                    // Float to unsigned integer
                    self.builder.build_float_to_unsigned_int(value, int_type, "fptoui_tmp")?.into()
                }
            }

            // If no supported type cast matches, return an error
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported type cast: {:?} to {:?}",
                    expr_value,
                    target_type
                ))
            }
        };

        // Return the casted value
        Ok(casted_value)
    }
}
