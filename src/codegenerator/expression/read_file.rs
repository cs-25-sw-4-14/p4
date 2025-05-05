// read_file.rs
use crate::ast::{Expression, Type};
use anyhow::Result;
use inkwell::values::{BasicValueEnum, PointerValue};

pub fn compile<'ctx>(gen: &mut CodeGenerator<'ctx>, read_expr: &Expression::Read) -> Result<BasicValueEnum<'ctx>> {
    // Compile the path (string) to be read
    let path_ir = gen.compile_expression(&read_expr.value)?;

    // Assuming you have an existing LLVM IR function for reading from a file
    let read_function = gen.module.get_function("read_file")
        .ok_or(anyhow::anyhow!("read_file function not found"))?;

    // Generate the call to read the file
    let read_result = gen.builder.build_call(read_function, &[path_ir.into()], "read_call");

    // Assuming `read_file` returns a string, extract it
    Ok(read_result.try_as_basic_value().left().unwrap())
}
