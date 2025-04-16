use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};
use inkwell::types::BasicTypeEnum;
use inkwell::IntPredicate;
use inkwell::AddressSpace;

use std::collections::HashMap;
use std::error::Error;
use anyhow::Result;

use crate::ast::{Program, TopLevelDefinition, Type};

pub mod function;
pub mod structs;
pub mod statement;
pub mod expression;

pub struct CodeGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub variables: HashMap<String, (BasicTypeEnum<'ctx>, PointerValue<'ctx>)>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Self {
        let builder = context.create_builder();
        CodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
        }
    }
    
    // Entry point for code generation: process all top-level definitions.
    pub fn compile_program(&mut self, program: &Program) -> Result<()> {
        for def in &program.definitions {
            match def {
                TopLevelDefinition::Function(func_def) => {
                    self.compile_function(&func_def)?;
                },
                TopLevelDefinition::Struct(struct_def) => {
                    self.compile_struct(&struct_def)?;
                },
            }
        }
        Ok(())
    }

    // Helper function to get an LLVM basic type from your AST Type
    pub fn get_basic_type(&self, ast_type: &Type) -> BasicTypeEnum<'ctx> {
        match ast_type {
            Type::Bool => self.context.bool_type().into(),
            Type::U8 => self.context.i8_type().into(),
            Type::U16 => self.context.i16_type().into(),
            Type::U32 => self.context.i32_type().into(),
            Type::U64 => self.context.i64_type().into(),
            Type::I8 => self.context.i8_type().into(),
            Type::I16 => self.context.i16_type().into(),
            Type::I32 => self.context.i32_type().into(),
            Type::I64 => self.context.i64_type().into(),
            Type::FP16 => self.context.f16_type().into(),
            Type::FP32 => self.context.f32_type().into(),
            Type::FP64 => self.context.f64_type().into(),
            Type::Char => self.context.i8_type().into(),
            _ => panic!("Unsupported type: {:?}", ast_type),
        }
    }
}
