// write_file.rs
use crate::ast::{Expression, Type};
use anyhow::Result;
use inkwell::values::{BasicValueEnum, PointerValue};

pub fn compile<'ctx>(gen: &mut CodeGenerator<'ctx>, write_expr: &Expression::Write) -> Result<BasicValueEnum<'ctx>> {
    // Compile the value (string to be written)
    let value_ir = gen.compile_expression(&write_expr.value)?;

    // Compile the target (file path)
    let target_ir = gen.compile_expression(&write_expr.target)?;

    // Assuming you have an existing LLVM IR function for writing to a file
    let write_function = gen.module.get_function("write_file")
        .ok_or(anyhow::anyhow!("write_file function not found"))?;

    // Generate the call to write to the file
    gen.builder.build_call(write_function, &[value_ir.into(), target_ir.into()], "write_call");

    Ok(value_ir) // You can return the value IR or any other relevant information
}
