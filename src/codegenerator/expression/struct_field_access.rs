use super::CodeGenerator;
use crate::ast::{StructFieldAccess, TopLevelDefinition, Expression, Type};
use anyhow::Result;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::BasicValueEnum;

impl<'ctx> CodeGenerator<'ctx> {
    pub fn compile_struct_field_access(
        &mut self,
        struct_field_access: &StructFieldAccess,
    ) -> Result<BasicValueEnum<'ctx>> {
       
        let struct_val = self.compile_expression(&struct_field_access.struct_expr)?;
        let struct_ptr = match struct_val {
            BasicValueEnum::PointerValue(p) => p,
            _ => return Err(anyhow::anyhow!("Expected struct to be a pointer")),
        };

        
        let struct_type_name = match &*struct_field_access.struct_expr {
            Expression::Variable(var_name) => {
                match self.variables.get(var_name) {
                    Some((type_enum, _)) => self.get_struct_name(type_enum)
                        .ok_or_else(|| anyhow::anyhow!("Variable '{}' is not a struct", var_name))?,
                    None => return Err(anyhow::anyhow!("Variable '{}' not found", var_name)),
                }
            }
            _ => return Err(anyhow::anyhow!("Unsupported struct expression")),
        };

        
        let field_index = self.program.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Program not initialized"))?
            .definitions.iter()
            .filter_map(|def| {
                if let TopLevelDefinition::Struct(def) = def {
                    (def.name == struct_type_name).then_some(def)
                } else {
                    None
                }
            })
            .next()
            .ok_or_else(|| anyhow::anyhow!("Struct '{}' not found", struct_type_name))?
            .fields.iter()
            .position(|f| f.name == struct_field_access.field_name)
            .ok_or_else(|| anyhow::anyhow!("Field '{}' not found", struct_field_access.field_name))?;

        // Get the struct type from the pointed struct
        let struct_type = self.module.get_struct_type(&struct_type_name)
            .ok_or_else(|| anyhow::anyhow!("Struct type '{}' not found in module", struct_type_name))?;
       
        // Build struct GEP with the correct struct type
        let field_ptr = self.builder.build_struct_gep(
            struct_type,
            struct_ptr,
            field_index as u32,
            "field_ptr",
        )?;

        
        let field_ast_type = self.program.as_ref()
            .unwrap()
            .definitions.iter()
            .filter_map(|def| {
                if let TopLevelDefinition::Struct(def) = def {
                    (def.name == struct_type_name).then_some(&def.fields[field_index].field_type)
                } else {
                    None
                }
            })
            .next()
            .ok_or_else(|| anyhow::anyhow!("Could not determine field type"))?;

        let llvm_field_type = self.get_llvm_type(field_ast_type)?;
        
        let field_value = self.builder.build_load(llvm_field_type, field_ptr ,"field_value")?;

        Ok(field_value)
    }

    fn get_struct_name(&self, type_enum: &BasicTypeEnum<'ctx>) -> Option<String> {
        if let BasicTypeEnum::StructType(struct_type) = type_enum {
            struct_type.get_name().map(|name| String::from_utf8_lossy(name.to_bytes()).into_owned())
        } else {
            None
        }
    }

    fn get_llvm_type(&self, ast_type: &Type) -> Result<BasicTypeEnum<'ctx>> {
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
            Type::String => Ok(self.context.ptr_type(Default::default()).as_basic_type_enum()),
            Type::Struct(name) => {
                self.module.get_struct_type(name)
                    .map(BasicTypeEnum::StructType)
                    .ok_or_else(|| anyhow::anyhow!("Struct type '{}' not found", name))
            }
            Type::Array(elem_type, size) => {
                let elem_type = self.get_llvm_type(elem_type)?;
                match elem_type {
                    BasicTypeEnum::ArrayType(_) => Err(anyhow::anyhow!("Nested arrays not supported")),
                    BasicTypeEnum::FloatType(ft) => Ok(ft.array_type(*size as u32).as_basic_type_enum()),
                    BasicTypeEnum::IntType(it) => Ok(it.array_type(*size as u32).as_basic_type_enum()),
                    BasicTypeEnum::PointerType(pt) => Ok(pt.array_type(*size as u32).as_basic_type_enum()),
                    BasicTypeEnum::StructType(st) => Ok(st.array_type(*size as u32).as_basic_type_enum()),
                    _ => Err(anyhow::anyhow!("Unsupported element type")),
                }
            }
            Type::Vector(_) => Err(anyhow::anyhow!("Vector types not supported yet")),
        }
    }
}