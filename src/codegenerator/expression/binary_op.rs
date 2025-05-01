use super::CodeGenerator;
use crate::ast::{BinaryOperation, BinaryOperator};
use anyhow::{Result, anyhow};
use inkwell::IntPredicate;
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_binary_op(&mut self, op: &BinaryOperation) -> Result<BasicValueEnum<'ctx>> {
        let left = self.compile_expression(&op.left)?;
        let right = self.compile_expression(&op.right)?;

        if left.get_type() != right.get_type() {
            return Err(anyhow!("Mismatched types in binary operation"));
        }

        match (left, right) {
            // all binary operations where both operands are integers
            (BasicValueEnum::IntValue(lhs), BasicValueEnum::IntValue(rhs)) => match op.operator {
                BinaryOperator::Add => {
                    let result = self.builder.build_int_add(lhs, rhs, "int_add")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::Subtract => {
                    let result = self.builder.build_int_sub(lhs, rhs, "int_sub")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::Multiply => {
                    let result = self.builder.build_int_mul(lhs, rhs, "int_mul")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::Divide => {
                    let result = self.builder.build_int_signed_div(lhs, rhs, "int_div")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::Modulus => {
                    let result = self.builder.build_int_signed_rem(lhs, rhs, "int_rem")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::Equal => {
                    let result =
                        self.builder
                            .build_int_compare(IntPredicate::EQ, lhs, rhs, "int_eq")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::NotEqual => {
                    let result =
                        self.builder
                            .build_int_compare(IntPredicate::NE, lhs, rhs, "int_ne")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::LessThan => {
                    let result =
                        self.builder
                            .build_int_compare(IntPredicate::SLT, lhs, rhs, "int_lt")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::GreaterThan => {
                    let result =
                        self.builder
                            .build_int_compare(IntPredicate::SGT, lhs, rhs, "int_gt")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::LessThanOrEqual => {
                    let result =
                        self.builder
                            .build_int_compare(IntPredicate::SLE, lhs, rhs, "int_le")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::GreaterThanOrEqual => {
                    let result =
                        self.builder
                            .build_int_compare(IntPredicate::SGE, lhs, rhs, "int_ge")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::And => {
                    let result = self.builder.build_and(lhs, rhs, "int_and")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
                BinaryOperator::Or => {
                    let result = self.builder.build_or(lhs, rhs, "int_or")?;
                    Ok(BasicValueEnum::IntValue(result))
                }
            },
            // all binary operations where both operands are floating point numbers
            (BasicValueEnum::FloatValue(lhs), BasicValueEnum::FloatValue(rhs)) => {
                match op.operator {
                    BinaryOperator::Add => {
                        let result = self.builder.build_float_add(lhs, rhs, "float_add")?;
                        Ok(BasicValueEnum::FloatValue(result))
                    }
                    BinaryOperator::Subtract => {
                        let result = self.builder.build_float_sub(lhs, rhs, "float_sub")?;
                        Ok(BasicValueEnum::FloatValue(result))
                    }
                    BinaryOperator::Multiply => {
                        let result = self.builder.build_float_mul(lhs, rhs, "float_mul")?;
                        Ok(BasicValueEnum::FloatValue(result))
                    }
                    BinaryOperator::Divide => {
                        let result = self.builder.build_float_div(lhs, rhs, "float_div")?;
                        Ok(BasicValueEnum::FloatValue(result))
                    }
                    BinaryOperator::Equal => {
                        let result = self.builder.build_float_compare(
                            inkwell::FloatPredicate::OEQ,
                            lhs,
                            rhs,
                            "float_eq",
                        )?;
                        Ok(BasicValueEnum::IntValue(result))
                    }
                    BinaryOperator::NotEqual => {
                        let result = self.builder.build_float_compare(
                            inkwell::FloatPredicate::ONE,
                            lhs,
                            rhs,
                            "float_ne",
                        )?;
                        Ok(BasicValueEnum::IntValue(result))
                    }
                    BinaryOperator::LessThan => {
                        let result = self.builder.build_float_compare(
                            inkwell::FloatPredicate::OLT,
                            lhs,
                            rhs,
                            "float_lt",
                        )?;
                        Ok(BasicValueEnum::IntValue(result))
                    }
                    BinaryOperator::GreaterThan => {
                        let result = self.builder.build_float_compare(
                            inkwell::FloatPredicate::OGT,
                            lhs,
                            rhs,
                            "float_gt",
                        )?;
                        Ok(BasicValueEnum::IntValue(result))
                    }
                    BinaryOperator::LessThanOrEqual => {
                        let result = self.builder.build_float_compare(
                            inkwell::FloatPredicate::OLE,
                            lhs,
                            rhs,
                            "float_le",
                        )?;
                        Ok(BasicValueEnum::IntValue(result))
                    }
                    BinaryOperator::GreaterThanOrEqual => {
                        let result = self.builder.build_float_compare(
                            inkwell::FloatPredicate::OGE,
                            lhs,
                            rhs,
                            "float_ge",
                        )?;
                        Ok(BasicValueEnum::IntValue(result))
                    }
                    BinaryOperator::Modulus => {
                        Err(anyhow!(
                            "Modulus operator not supported for floating point values"
                        ))
                    }
                    BinaryOperator::And => {
                        Err(anyhow!(
                            "And operator not supported for floating point values"
                        ))
                    }
                    BinaryOperator::Or => {
                        Err(anyhow!(
                            "Or operator not supported for floating point values"
                        ))
                    }
                }
            }

            _ => Err(anyhow!("Unsupported value types for binary operation")),
        }
    }
}
