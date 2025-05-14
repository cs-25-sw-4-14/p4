use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, PointerValue};

use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::ast::{Program, StructDef, TopLevelDefinition, Type};

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
    pub current_function: Option<inkwell::values::FunctionValue<'ctx>>,
    pub break_target_stack: Vec<inkwell::basic_block::BasicBlock<'ctx>>,
    pub continue_target_stack: Vec<inkwell::basic_block::BasicBlock<'ctx>>,
    pub struct_definitions: HashMap<String, StructDef>,
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
        module.add_function("fopen", fopen_type, Some(Linkage::External));

        // fprintf: int fprintf(FILE* stream, const char* format, ...);
        let fprintf_type = i32_type.fn_type(&[file_ptr_type.into(), i8_ptr_type.into()], true);
        module.add_function("fprintf", fprintf_type, Some(Linkage::External));

        // fclose: int fclose(FILE* stream);
        let fclose_type = i32_type.fn_type(&[file_ptr_type.into()], false);
        module.add_function("fclose", fclose_type, Some(Linkage::External));

        // fseek: int fseek(FILE* stream, long offset, int whence);
        let fseek_type = i32_type.fn_type(
            &[file_ptr_type.into(), i64_type.into(), i32_type.into()],
            false,
        );
        module.add_function("fseek", fseek_type, Some(Linkage::External));

        // ftell: long ftell(FILE* stream);
        let ftell_type = i64_type.fn_type(&[file_ptr_type.into()], false);
        module.add_function("ftell", ftell_type, Some(Linkage::External));

        // fread: size_t fread(void* ptr, size_t size, size_t count, FILE* stream);
        let fread_type = i64_type.fn_type(
            &[
                i8_ptr_type.into(),
                i64_type.into(),
                i64_type.into(),
                file_ptr_type.into(),
            ],
            false,
        );
        module.add_function("fread", fread_type, Some(Linkage::External));

        // malloc: void* malloc(size_t size);
        let malloc_type = i8_ptr_type.fn_type(&[i64_type.into()], false);
        module.add_function("malloc", malloc_type, Some(Linkage::External));

        CodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
            last_value: None,
            current_function: None,
            break_target_stack: Vec::new(),
            continue_target_stack: Vec::new(),
            struct_definitions: HashMap::new(),
        }
    }

    // Helper function to get a global string pointer for string literals
    // This function creates a global string in the LLVM module and returns a pointer to it.
    fn get_global_string_ptr(
        &self,
        string_literal: &str,
        name: &str,
    ) -> Result<PointerValue<'ctx>> {
        let c_string = format!("{}\0", string_literal);
        let bytes = c_string.as_bytes();
        let global_str_type = self.context.i8_type().array_type(bytes.len() as u32);
        let global_str =
            self.module
                .add_global(global_str_type, Some(AddressSpace::default()), name);
        global_str.set_linkage(Linkage::Private);
        global_str.set_initializer(&self.context.const_string(bytes, false));
        global_str.set_constant(true);

        let i8_ptr_type = self.context.ptr_type(AddressSpace::default());
        self.builder
            .build_pointer_cast(
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
                    self.struct_definitions
                        .insert(struct_def.name.clone(), struct_def.clone());
                }
                // TODO: Handle enums
                TopLevelDefinition::Enum(_) => todo!(),
            }
        }
        Ok(())
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
            Type::Custom(name) => match self.module.get_struct_type(name) {
                Some(struct_ty) => Ok(struct_ty.as_basic_type_enum()),
                None => Err(anyhow!("Struct type '{}' not found", name)),
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
            Type::Vector(elem_type) => {
                let elem_llvm_type = self.get_llvm_type(elem_type)?;
                match elem_llvm_type {
                    BasicTypeEnum::FloatType(ft) => {
                        Ok(ft.vec_type(1).as_basic_type_enum())
                    }
                    BasicTypeEnum::IntType(it) => {
                        Ok(it.vec_type(1).as_basic_type_enum())
                    }
                    BasicTypeEnum::PointerType(pt) => {
                        Ok(pt.vec_type(1).as_basic_type_enum())
                    }
                    BasicTypeEnum::StructType(_) => {
                        Ok(self.context.ptr_type(AddressSpace::default()).vec_type(0).as_basic_type_enum())
                    }
                    _ => Err(anyhow::anyhow!("Unsupported vector element type")),
                }
            }
        }
    }
}
