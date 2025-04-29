use super::CodeGenerator;
use crate::ast::{StructDef, Type};
use anyhow::Result;
use inkwell::types::{BasicTypeEnum, BasicType};

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct(&mut self, struct_def: &StructDef) -> Result<()> {
        // Store the program for reference if not already stored
        if self.program.is_none() {
            return Err(anyhow::anyhow!("Program not loaded"));
        }

        // Create an opaque struct type first (with no body defined yet)
        let struct_type = self.context.opaque_struct_type(&struct_def.name);
        
        // Convert field types to LLVM types
        let field_types: Result<Vec<BasicTypeEnum<'ctx>>> = struct_def
            .fields
            .iter()
            .map(|field| {
                match &field.field_type {
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
                    Type::String => Ok(self.context.ptr_type(Default::default()).as_basic_type_enum()),
                    Type::Struct(name) => {
                        // Handle nested struct types
                        match self.module.get_struct_type(name) {
                            Some(st) => Ok(st.as_basic_type_enum()),
                            None => Err(anyhow::anyhow!("Struct {} not defined before use", name))
                        }
                    }
                    Type::Array(elem_type, size) => {
                        // Handle array types
                        let elem_llvm_type = match **elem_type {
                            Type::U8 => self.context.i8_type().as_basic_type_enum(),
                            Type::U16 => self.context.i16_type().as_basic_type_enum(),
                            Type::U32 => self.context.i32_type().as_basic_type_enum(),
                            Type::U64 => self.context.i64_type().as_basic_type_enum(),
                            Type::I8 => self.context.i8_type().as_basic_type_enum(),
                            Type::I16 => self.context.i16_type().as_basic_type_enum(),
                            Type::I32 => self.context.i32_type().as_basic_type_enum(),
                            Type::I64 => self.context.i64_type().as_basic_type_enum(),
                            Type::FP8 | Type::FP16 => self.context.f16_type().as_basic_type_enum(),
                            Type::FP32 => self.context.f32_type().as_basic_type_enum(),
                            Type::FP64 => self.context.f64_type().as_basic_type_enum(),
                            Type::Bool => self.context.bool_type().as_basic_type_enum(),
                            Type::Char => self.context.i8_type().as_basic_type_enum(),
                            _ => return Err(anyhow::anyhow!("Unsupported array element type"))
                        };
                        
                        match elem_llvm_type {
                            BasicTypeEnum::FloatType(ft) => Ok(ft.array_type(*size as u32).as_basic_type_enum()),
                            BasicTypeEnum::IntType(it) => Ok(it.array_type(*size as u32).as_basic_type_enum()),
                            BasicTypeEnum::PointerType(pt) => Ok(pt.array_type(*size as u32).as_basic_type_enum()),
                            BasicTypeEnum::StructType(st) => Ok(st.array_type(*size as u32).as_basic_type_enum()),
                            _ => Err(anyhow::anyhow!("Unsupported array element type"))
                        }
                    }
                    Type::Vector(_) => Err(anyhow::anyhow!("Vector types not supported yet"))
                }
            })
            .collect();

        // Set the struct body with the field types
        let field_types = field_types?;
        struct_type.set_body(&field_types, false);

        Ok(())
    }
}