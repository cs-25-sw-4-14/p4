use super::CodeGenerator;
use crate::ast::VectorLiteral;
use anyhow::Result;
use inkwell::{types::BasicType, values::BasicValueEnum};


impl <'ctx> CodeGenerator<'ctx> {
    pub fn compile_array_literal(&mut self,
    vector_literal: &Box<VectorLiteral>) -> Result<BasicValueEnum<'ctx>>{

    }



}