use super::CodeGenerator;
use crate::ast::Literal;
use crate::ast::{Expression, Type, TypeCast};
use anyhow::{Result, anyhow};
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn get_float_bit_width(&self, float_type: inkwell::types::FloatType<'ctx>) -> Result<u32> {
        if float_type.eq(&self.context.f16_type()) {
            Ok(16)
        } else if float_type.eq(&self.context.f32_type()) {
            Ok(32)
        } else if float_type.eq(&self.context.f64_type()) {
            Ok(64)
        } else {
            Err(anyhow!("Unknown floating-point type"))
        }
    }
}

impl<'ctx> CodeGenerator<'ctx> {
    // Function to compile a type cast expression
    pub fn compile_type_cast(&mut self, type_cast: &TypeCast) -> Result<BasicValueEnum<'ctx>> {
        // Compile the expression to a value
        let expr_value = self.compile_expression(&type_cast.expression)?;
        // Get the target type for the cast
        let target_type = self.get_llvm_type(&type_cast.target_type)?;

        // Match on the types of expression and target type
        let casted_value = match (expr_value, target_type) {
            // Integer to Integer casting
            (BasicValueEnum::IntValue(value), BasicTypeEnum::IntType(int_type)) => {
                let src_bits = value.get_type().get_bit_width();
                let tgt_bits = int_type.get_bit_width();
                // Determine if the target type is signed or unsigned
                let is_signed = matches!(
                    type_cast.target_type,
                    Type::I8 | Type::I16 | Type::I32 | Type::I64
                );

                match src_bits.cmp(&tgt_bits) {
                    std::cmp::Ordering::Less if is_signed => self
                        .builder
                        .build_int_s_extend(value, int_type, "sext_tmp")?
                        .into(),
                    std::cmp::Ordering::Less => self
                        .builder
                        .build_int_z_extend(value, int_type, "zext_tmp")?
                        .into(),
                    std::cmp::Ordering::Greater => self
                        .builder
                        .build_int_truncate(value, int_type, "trunc_tmp")?
                        .into(),
                    std::cmp::Ordering::Equal => value.into(),
                }
            }

            // Float to Float casting
            (BasicValueEnum::FloatValue(value), BasicTypeEnum::FloatType(float_type)) => {
                let src_bits = self.get_float_bit_width(value.get_type()).unwrap();
                let tgt_bits = self.get_float_bit_width(float_type).unwrap();

                match src_bits.cmp(&tgt_bits) {
                    std::cmp::Ordering::Less => self
                        .builder
                        .build_float_ext(value, float_type, "fext_tmp")?
                        .into(),
                    std::cmp::Ordering::Greater => self
                        .builder
                        .build_float_trunc(value, float_type, "ftrunc_tmp")?
                        .into(),
                    std::cmp::Ordering::Equal => value.into(),
                }
            }

            // Integer to Float casting
            (BasicValueEnum::IntValue(value), BasicTypeEnum::FloatType(float_type)) => {
                // Check if the integer is signed
                let is_signed = matches!(
                    type_cast.expression.as_ref(),
                    Expression::Literal(Literal::Int(_))
                );
                if is_signed {
                    // Signed integer to float
                    self.builder
                        .build_signed_int_to_float(value, float_type, "sitofp_tmp")?
                        .into()
                } else {
                    // Unsigned integer to float
                    self.builder
                        .build_unsigned_int_to_float(value, float_type, "uitofp_tmp")?
                        .into()
                }
            }

            // Float to Integer casting
            (BasicValueEnum::FloatValue(value), BasicTypeEnum::IntType(int_type)) => {
                // Check if the target type is signed or unsigned
                let is_signed = matches!(
                    type_cast.target_type,
                    Type::I8 | Type::I16 | Type::I32 | Type::I64
                );
                if is_signed {
                    // Float to signed integer
                    self.builder
                        .build_float_to_signed_int(value, int_type, "fptosi_tmp")?
                        .into()
                } else {
                    // Float to unsigned integer
                    self.builder
                        .build_float_to_unsigned_int(value, int_type, "fptoui_tmp")?
                        .into()
                }
            }

            // String to char array casting
            (BasicValueEnum::PointerValue(ptr), BasicTypeEnum::ArrayType(array_type)) => {
                if let BasicTypeEnum::IntType(int_type) = array_type.get_element_type() {
                    if int_type.get_bit_width() == 8 {
                        ptr.into()
                    } else {
                        return Err(anyhow::anyhow!("Cannot cast to non-char array type"));
                    }
                } else {
                    return Err(anyhow::anyhow!("Cannot cast to non-array type"));
                }
            }

            // If no supported type cast matches, return an error
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported type cast: {:?} to {:?}",
                    expr_value,
                    target_type
                ));
            }
        };

        // Return the casted value
        Ok(casted_value)
    }
}
