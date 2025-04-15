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

use crate::ast::{Program, TopLevelDefinition};

pub mod function;
pub mod structs;
pub mod statement;
pub mod expression;

pub struct CodeGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub variables: HashMap<String, PointerValue<'ctx>>,
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
}
