use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, PointerValue};

use anyhow::Result;
use std::collections::HashMap;

use crate::ast::{Program, TopLevelDefinition, Type};

pub mod expression;
pub mod function;
pub mod statement;
pub mod structs;

// Represents a variable in our scope system
#[derive(Clone)]
pub struct ScopedVariable<'ctx> {
    pub name: String,
    pub var_type: BasicTypeEnum<'ctx>,
    pub ptr: PointerValue<'ctx>,
    pub needs_free: bool, // True for heap-allocated variables
}

// Represents a scope in our system
#[derive(Clone)]
pub struct Scope<'ctx> {
    pub variables: HashMap<String, (BasicTypeEnum<'ctx>, PointerValue<'ctx>)>,
    pub to_free: Vec<ScopedVariable<'ctx>>,
}

impl<'ctx> Scope<'ctx> {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
            to_free: Vec::new(),
        }
    }
}

pub struct CodeGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub variables: HashMap<String, (BasicTypeEnum<'ctx>, PointerValue<'ctx>)>,
    pub scope_stack: Vec<Scope<'ctx>>,
    pub current_function: Option<inkwell::values::FunctionValue<'ctx>>,
    pub last_value: Option<BasicValueEnum<'ctx>>,
    pub program: Option<Program>,
}

impl<'ctx> CodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module: Module<'ctx>) -> Self {
        let builder = context.create_builder();

        let i8_ptr_type = context.ptr_type(inkwell::AddressSpace::default());
        let printf_type = context.i32_type().fn_type(&[i8_ptr_type.into()], true);
        module.add_function("printf", printf_type, None);

        // Create malloc and free functions for heap memory management
        let i8_ptr = context.ptr_type(AddressSpace::default());
        let malloc_type = i8_ptr.fn_type(&[context.i64_type().into()], false);
        module.add_function("malloc", malloc_type, None);

        let free_type = context.void_type()
            .fn_type(&[i8_ptr.into()], false);
        module.add_function("free", free_type, None);

        CodeGenerator {
            context,
            module,
            builder,
            variables: HashMap::new(),
            scope_stack: vec![Scope::new()], // Initialize with global scope
            current_function: None,
            last_value: None,
            program: None,
        }
    }
    
    // Add methods for scope management
    pub fn enter_scope(&mut self) {
        // Create a new scope and push it onto the stack
        self.scope_stack.push(Scope::new());
    }
    
    pub fn exit_scope(&mut self) -> Result<()> {
        // Pop the current scope
        if let Some(scope) = self.scope_stack.pop() {
            // Free any heap-allocated variables in this scope
            self.free_scope_variables(&scope)?;
            
            // Merge the parent scope's variables back into our flat variables map
            if let Some(parent_scope) = self.scope_stack.last() {
                self.variables = parent_scope.variables.clone();
            } else {
                // We're back to global scope, clear the variables
                self.variables.clear();
                // Push a new global scope
                self.scope_stack.push(Scope::new());
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("No scope to exit"))
        }
    }
    
    // Helper method to free all variables in a scope
    fn free_scope_variables(&mut self, scope: &Scope<'ctx>) -> Result<()> {
        let free_fn = self.module.get_function("free")
            .ok_or_else(|| anyhow::anyhow!("Could not find free function"))?;
            
        // Free any heap-allocated variables
        for var in &scope.to_free {
            if var.needs_free {
                // Cast the pointer to i8* as required by free
                let i8_ptr_type = self.context.ptr_type(AddressSpace::default());
                let cast_ptr = self.builder.build_bit_cast(
                    var.ptr, 
                    i8_ptr_type, 
                    &format!("{}_free_cast", var.name)
                )?;
                
                // Call free
                self.builder.build_call(
                    free_fn,
                    &[inkwell::values::BasicMetadataValueEnum::PointerValue(cast_ptr)],
                    &format!("{}_free", var.name)
                )?;
            }
        }
        
        Ok(())
    }
    
    // Register a variable that needs to be freed when it goes out of scope
    pub fn register_heap_allocation(
        &mut self, 
        name: String,
        var_type: BasicTypeEnum<'ctx>,
        ptr: PointerValue<'ctx>
    ) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.to_free.push(ScopedVariable {
                name,
                var_type,
                ptr,
                needs_free: true,
            });
        }
    }

    // Entry point for code generation: process all top-level definitions.
    pub fn compile_program(&mut self, program: &Program) -> Result<()> {
        // Store the program for use in struct compilation
        self.program = Some(program.clone());
        
        // First pass: compile all struct definitions to create types
        for def in &program.definitions {
            if let TopLevelDefinition::Struct(struct_def) = def {
                self.compile_struct(struct_def)?;
            }
        }
        
        // Second pass: compile all function definitions
        for def in &program.definitions {
            if let TopLevelDefinition::Function(func_def) = def {
                self.compile_function(func_def)?;
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
    // Helper function to extract a struct type name from a BasicTypeEnum
    pub fn get_struct_name(&self, type_enum: &BasicTypeEnum<'ctx>) -> Option<String> {
        if let BasicTypeEnum::StructType(struct_type) = type_enum {
            struct_type.get_name().map(|name| String::from_utf8_lossy(name.to_bytes()).into_owned())
        } else {
            None
        }
    }
    
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
            Type::String => {
                // Create a pointer to i8 for String representation using the recommended method
                let char_ptr_type = self.context.ptr_type(AddressSpace::default());
                Ok(char_ptr_type.as_basic_type_enum())
            },
            Type::Struct(name) => match self.module.get_struct_type(name) {
                Some(struct_ty) => Ok(struct_ty.as_basic_type_enum()),
                None => Err(anyhow::anyhow!("Struct type '{}' not found", name)),
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
            }
            Type::Vector(_) => Err(anyhow::anyhow!("Vector types not supported yet")),
        }
    }
}
