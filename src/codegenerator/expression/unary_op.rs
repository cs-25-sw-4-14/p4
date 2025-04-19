use super::CodeGenerator;
use crate::ast::{UnaryOperation, UnaryOperator};
use anyhow::{Result, anyhow};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_unary_op(&mut self, op: &UnaryOperation) -> Result<BasicValueEnum<'ctx>> {
        let operand = self.compile_expression(&op.operand)?;

        match op.operator {
            // negates a numeric value (integer or float)
            UnaryOperator::Negate => {
                match operand {
                    // integer negation
                    BasicValueEnum::IntValue(int_val) => {
                        let zero = int_val.get_type().const_zero();
                        let result = self.builder.build_int_sub(zero, int_val, "int_neg")?;
                        Ok(BasicValueEnum::IntValue(result))
                    }
                    // float negation
                    BasicValueEnum::FloatValue(float_val) => {
                        let result = self.builder.build_float_neg(float_val, "float_neg")?;
                        Ok(BasicValueEnum::FloatValue(result))
                    }

                    _ => Err(anyhow!("Cannot negate non-numeric type")),
                }
            }

            // logical NOT operation
            UnaryOperator::Not => {
                match operand {
                    // boolean NOT
                    BasicValueEnum::IntValue(int_val) => {
                        if int_val.get_type().get_bit_width() == 1 {
                            // XOR with 1 (true) - equivalent to logical NOT
                            let true_const = int_val.get_type().const_all_ones();
                            let result = self.builder.build_xor(int_val, true_const, "bool_not")?;
                            Ok(BasicValueEnum::IntValue(result))
                        } else {
                            Err(anyhow!("Logical NOT can only be applied to boolean values"))
                        }
                    }

                    _ => Err(anyhow!("Logical NOT can only be applied to boolean values")),
                }
            }
        }
    }
}
