use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, PointerValue};

use anyhow::Result;
use std::collections::HashMap;

use crate::ast::{Program, TopLevelDefinition, Type};

pub mod controlflow;
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
    pub current_function: Option<inkwell::values::FunctionValue<'ctx>>,
    pub break_target_stack: Vec<inkwell::basic_block::BasicBlock<'ctx>>,
    pub continue_target_stack: Vec<inkwell::basic_block::BasicBlock<'ctx>>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Self {
        let builder = context.create_builder();

        let i8_ptr_type = context.ptr_type(inkwell::AddressSpace::default());
        let printf_type = context.i32_type().fn_type(&[i8_ptr_type.into()], true);
        module.add_function("printf", printf_type, None);

        CodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
            last_value: None,
            program: None,
            current_function: None,
            break_target_stack: Vec::new(),
            continue_target_stack: Vec::new(),
        }
    }

    // Entry point for code generation: process all top-level definitions.
    pub fn compile_program(&mut self, program: &Program) -> Result<()> {
        for def in &program.definitions {
            match def {
                TopLevelDefinition::Function(func_def) => {
                    self.compile_function(func_def)?;
                }
                TopLevelDefinition::Struct(struct_def) => {
                    self.compile_struct(struct_def)?;
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
            Type::F16 => self.context.f16_type().into(),
            Type::F32 => self.context.f32_type().into(),
            Type::F64 => self.context.f64_type().into(),
            Type::Char => self.context.i8_type().into(),
            // Type::String => self
            //     .context
            //     .i8_type()
            //     .ptr_type(AddressSpace::Generic)
            //     .into(), // Pointer to i8 (C-style string)
            // Type::Struct(name) => {
            //     // Ensure struct type exists
            //     let struct_type = self
            //         .module
            //         .get_struct_type(name)
            //         .expect(&format!("Struct type '{}' not found", name));
            //     struct_type.into()
            // }
            Type::Array(element_type, size) => {
                let llvm_element_type = self.get_basic_type(element_type);
                // Handle array size overflow
                if *size > u32::MAX as u64 {
                    panic!(
                        "Array size {} exceeds maximum supported size {}",
                        size,
                        u32::MAX
                    );
                }
                llvm_element_type.array_type(*size as u32).into()
            }
            // Type::Vector(element_type) => {
            //     let llvm_element_type = self.get_basic_type(element_type);
            //     llvm_element_type.vector_type(4).into() // Example: 4 elements
            // }
            _ => panic!("Unsupported type: {:?}", ast_type), // Catch-all for unsupported types
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
            Type::F16 => Ok(self.context.f16_type().as_basic_type_enum()),
            Type::F32 => Ok(self.context.f32_type().as_basic_type_enum()),
            Type::F64 => Ok(self.context.f64_type().as_basic_type_enum()),
            Type::Bool => Ok(self.context.bool_type().as_basic_type_enum()),
            Type::Reference(_, _) => todo!(),
            Type::String => Ok(self
                .context
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum()),
            Type::Struct(name) => match self.module.get_struct_type(name) {
                Some(struct_ty) => Ok(struct_ty.as_basic_type_enum()),
                None => Err(anyhow::anyhow!("Struct type '{}' not found", name)),
            },
            Type::Array(elem_type, size) => {
                let elem_llvm_type = self.get_llvm_type(elem_type)?;
                match elem_llvm_type {
                    BasicTypeEnum::FloatType(ft) => {
                        Ok(ft.array_type(*size as u32).as_basic_type_enum())
                    }
                    BasicTypeEnum::IntType(it) => {
                        Ok(it.array_type(*size as u32).as_basic_type_enum())
                    }
                    BasicTypeEnum::PointerType(pt) => {
                        Ok(pt.array_type(*size as u32).as_basic_type_enum())
                    }
                    BasicTypeEnum::StructType(st) => {
                        Ok(st.array_type(*size as u32).as_basic_type_enum())
                    }
                    _ => Err(anyhow::anyhow!("Unsupported array element type")),
                }
            }
            Type::Vector(_, _) => Err(anyhow::anyhow!("Vector types not supported yet")),
        }
    }
}
