use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicTypeEnum, BasicType};
use inkwell::values::{BasicValueEnum, PointerValue};

use anyhow::Result;
use std::collections::HashMap;

use crate::ast::{Program, TopLevelDefinition, Type};

pub mod expression;
pub mod function;
pub mod statement;
pub mod structs;

pub struct CodeGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub variables: HashMap<String, (BasicTypeEnum<'ctx>, PointerValue<'ctx>)>,
    pub last_value: Option<BasicValueEnum<'ctx>>,
    pub program: Option<Program>,
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

    // Helper function to get an LLVM basic type from your AST Type (simple version, doesn't handle all types)
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
    
    // Comprehensive helper method to get an LLVM type from an AST type, with error handling
    pub fn get_llvm_type(&self, ast_type: &Type) -> Result<BasicTypeEnum<'ctx>> {
        match ast_type {
            Type::U8 | Type::Char => Ok(self.context.i8_type().as_basic_type_enum()),
            Type::U16 => Ok(self.context.i16_type().as_basic_type_enum()),
            Type::U32 => Ok(self.context.i32_type().as_basic_type_enum()),
            Type::U64 => Ok(self.context.i64_type().as_basic_type_enum()),
            Type::I8 => Ok(self.context.i8_type().as_basic_type_enum()),
            Type::I16 => Ok(self.context.i16_type().as_basic_type_enum()),
            Type::I32 => Ok(self.context.i32_type().as_basic_type_enum()),
            Type::I64 => Ok(self.context.i64_type().as_basic_type_enum()),
            Type::FP8 | Type::FP16 => Ok(self.context.f16_type().as_basic_type_enum()),
            Type::FP32 => Ok(self.context.f32_type().as_basic_type_enum()),
            Type::FP64 => Ok(self.context.f64_type().as_basic_type_enum()),
            Type::Bool => Ok(self.context.bool_type().as_basic_type_enum()),
            Type::String => Ok(self.context.ptr_type(AddressSpace::default()).as_basic_type_enum()),
            Type::Struct(name) => {
                match self.module.get_struct_type(name) {
                    Some(struct_ty) => Ok(struct_ty.as_basic_type_enum()),
                    None => Err(anyhow::anyhow!("Struct type '{}' not found", name)),
                }
            },
            Type::Array(elem_type, size) => {
                let elem_llvm_type = self.get_llvm_type(elem_type)?;
                match elem_llvm_type {
                    BasicTypeEnum::FloatType(ft) => Ok(ft.array_type(*size as u32).as_basic_type_enum()),
                    BasicTypeEnum::IntType(it) => Ok(it.array_type(*size as u32).as_basic_type_enum()),
                    BasicTypeEnum::PointerType(pt) => Ok(pt.array_type(*size as u32).as_basic_type_enum()),
                    BasicTypeEnum::StructType(st) => Ok(st.array_type(*size as u32).as_basic_type_enum()),
                    _ => Err(anyhow::anyhow!("Unsupported array element type")),
                }
            },
            Type::Vector(_) => Err(anyhow::anyhow!("Vector types not supported yet")),
        }
    }
}
