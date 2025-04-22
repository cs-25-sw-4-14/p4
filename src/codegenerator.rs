use inkwell::AddressSpace;
use inkwell::IntPredicate;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FunctionValue, PointerValue};

use anyhow::Result;
use std::collections::HashMap;
use std::error::Error;

use crate::ast::{Program, TopLevelDefinition, Type};

pub mod expression;
pub mod function;
pub mod statement;
pub mod structs;

pub struct CodeGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub variables: HashMap<String, (PointerValue<'ctx>, bool)>,
    pub last_value: Option<BasicValueEnum<'ctx>>,
    pub program: Option<Program>,
    pub function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Self {
        let builder = context.create_builder();
        CodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
            last_value: None,
            program: None,
        }
    }

    // Entry point for code generation: process all top-level definitions.
    pub fn compile_program(&mut self, program: &Program) -> Result<()> {
        for def in &program.definitions {
            match def {
                TopLevelDefinition::Function(func_def) => {
                    self.compile_function(&func_def)?;
                }
                TopLevelDefinition::Struct(struct_def) => {
                    self.compile_struct(&struct_def)?;
                }
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
            Type::String => self.context.i8_type().ptr_type(AddressSpace::Generic).into(), // Pointer to i8 (C-style string)
            Type::Struct(name) => {
                // Ensure struct type exists
                let struct_type = self.module.get_struct_type(name)
                    .expect(&format!("Struct type '{}' not found", name));
                struct_type.into()
            }
            Type::Array(element_type, size) => {
                let llvm_element_type = self.get_basic_type(element_type);
                // Handle array size overflow
                if *size > u32::MAX as u64 {
                    panic!("Array size {} exceeds maximum supported size {}", size, u32::MAX);
                }
                llvm_element_type.array_type(*size as u32).into()
            }
            Type::Vector(element_type) => {
                let llvm_element_type = self.get_basic_type(element_type);
                // If the vector length is dynamic, pass that as a parameter
                llvm_element_type.array_type(4).into() // Example: Fixed length of 4, adjust dynamically if needed
            }
    
            _ => panic!("Unsupported type: {:?}", ast_type), // Catch-all for unsupported types
        }
    }
}
