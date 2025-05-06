use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Module, Linkage};
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, PointerValue, FunctionValue};

use anyhow::{Result};
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
    
    fopen_func: FunctionValue<'ctx>,
    fprintf_func: FunctionValue<'ctx>,
    fclose_func: FunctionValue<'ctx>,
    fseek_func: FunctionValue<'ctx>,
    ftell_func: FunctionValue<'ctx>,
    fread_func: FunctionValue<'ctx>,
    malloc_func: FunctionValue<'ctx>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Self {
        let builder = context.create_builder();

        let i32_type = context.i32_type();
        let i64_type = context.i64_type();
        let i8_ptr_type = context.ptr_type(AddressSpace::default());
        let file_ptr_type = i8_ptr_type;

        let printf_type = context.i32_type().fn_type(&[i8_ptr_type.into()], true);
        module.add_function("printf", printf_type, None);

        // Defines the C standard library functions we will use
        // fopen: FILE* fopen(const char* filename, const char* mode);

        let fopen_type = file_ptr_type.fn_type(&[i8_ptr_type.into(), i8_ptr_type.into()], false);
        let fopen_func = module.add_function("fopen", fopen_type, Some(Linkage::External));

        // fprintf: int fprintf(FILE* stream, const char* format, ...);
        let fprintf_type = i32_type.fn_type(&[file_ptr_type.into(), i8_ptr_type.into()], true);
        let fprintf_func = module.add_function("fprintf", fprintf_type, Some(Linkage::External));

        // fclose: int fclose(FILE* stream);
        let fclose_type = i32_type.fn_type(&[file_ptr_type.into()], false);
        let fclose_func = module.add_function("fclose", fclose_type, Some(Linkage::External));

        // fseek: int fseek(FILE* stream, long offset, int whence);
        let fseek_type = i32_type.fn_type(&[file_ptr_type.into(), i64_type.into(), i32_type.into()], false);
        let fseek_func = module.add_function("fseek", fseek_type, Some(Linkage::External));

        // ftell: long ftell(FILE* stream);
        let ftell_type = i64_type.fn_type(&[file_ptr_type.into()], false);
        let ftell_func = module.add_function("ftell", ftell_type, Some(Linkage::External));

        // fread: size_t fread(void* ptr, size_t size, size_t count, FILE* stream);
        let fread_type = i64_type.fn_type(&[i8_ptr_type.into(), i64_type.into(), i64_type.into(), file_ptr_type.into()], false);
        let fread_func = module.add_function("fread", fread_type, Some(Linkage::External));

        // malloc: void* malloc(size_t size);
        let malloc_type = i8_ptr_type.fn_type(&[i64_type.into()], false);
        let malloc_func = module.add_function("malloc", malloc_type, Some(Linkage::External));

        // Initialize the CodeGenerator struct
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
            fprintf_func: fprintf_func,
            fopen_func: fopen_func,
            fclose_func: fclose_func,
            fseek_func: fseek_func,
            ftell_func: ftell_func,
            fread_func: fread_func,
            malloc_func: malloc_func,
        }
    }

    // Helper function to get a global string pointer for string literals
    // This function creates a global string in the LLVM module and returns a pointer to it.
    fn get_global_string_ptr(&self, string_literal: &str, name: &str) -> Result<PointerValue<'ctx>> {
        let c_string = format!("{}\0", string_literal);
        let bytes = c_string.as_bytes();
        let global_str_type = self.context.i8_type().array_type(bytes.len() as u32);
        let global_str = self.module.add_global(global_str_type, Some(AddressSpace::default()), name);
        global_str.set_linkage(Linkage::Private);
        global_str.set_initializer(&self.context.const_string(bytes, false));
        global_str.set_constant(true);
    
        let i8_ptr_type = self.context.ptr_type(AddressSpace::default());
        self.builder.build_pointer_cast(
            global_str.as_pointer_value(),
            i8_ptr_type,
            &format!("{}_ptr", name),
        )
        .map_err(anyhow::Error::new) // 
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
            Type::FP16 => self.context.f16_type().into(),
            Type::FP32 => self.context.f32_type().into(),
            Type::FP64 => self.context.f64_type().into(),
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
            //     // If the vector length is dynamic, pass that as a parameter
            //     llvm_element_type.array_type(4).into() // Example: Fixed length of 4, adjust dynamically if needed
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
            Type::FP16 => Ok(self.context.f16_type().as_basic_type_enum()),
            Type::FP32 => Ok(self.context.f32_type().as_basic_type_enum()),
            Type::FP64 => Ok(self.context.f64_type().as_basic_type_enum()),
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
            Type::Vector(_) => Err(anyhow::anyhow!("Vector types not supported yet")),
        }
    }
}
