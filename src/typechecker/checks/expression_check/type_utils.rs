use crate::ast::{Type, Literal};

/// Check if two types are compatible (the actual can be used where the expected is required)
pub fn types_match(actual: &Type, expected: &Type) -> bool {
    match (actual, expected) {
        // Exact type matches
        (Type::U8, Type::U8) | (Type::U16, Type::U16) | (Type::U32, Type::U32) | (Type::U64, Type::U64) |
        (Type::I8, Type::I8) | (Type::I16, Type::I16) | (Type::I32, Type::I32) | (Type::I64, Type::I64) |
        (Type::FP8, Type::FP8) | (Type::FP16, Type::FP16) | (Type::FP32, Type::FP32) | (Type::FP64, Type::FP64) |
        (Type::String, Type::String) | (Type::Char, Type::Char) | (Type::Bool, Type::Bool) => true,
        
        // Integer type compatibility (allow implicit narrowing if the types are compatible)
        (Type::I64, Type::I32) | (Type::I64, Type::I16) | (Type::I64, Type::I8) |
        (Type::I64, Type::U64) | (Type::I64, Type::U32) | (Type::I64, Type::U16) | (Type::I64, Type::U8) => true,
        
        (Type::I32, Type::I16) | (Type::I32, Type::I8) |
        (Type::I32, Type::U32) | (Type::I32, Type::U16) | (Type::I32, Type::U8) => true,
        
        (Type::I16, Type::I8) | (Type::I16, Type::U16) | (Type::I16, Type::U8) => true,
        
        (Type::U64, Type::U32) | (Type::U64, Type::U16) | (Type::U64, Type::U8) => true,
        (Type::U32, Type::U16) | (Type::U32, Type::U8) => true,
        (Type::U16, Type::U8) => true,
        
        // Custom types must match by name
        (Type::Custom(a), Type::Custom(b)) => a == b,
        
        // For array and vector types, recursively check element types
        (Type::Array(a_type, a_size), Type::Array(b_type, b_size)) => 
            a_size == b_size && types_match(a_type, b_type),
        (Type::Vector(a_type), Type::Vector(b_type)) => 
            types_match(a_type, b_type),
            
        // All other combinations are not matches
        _ => false,
    }
}

/// Check if a literal value can be converted to the specified type
pub fn can_convert_literal(literal: &Literal, target_type: &Type) -> bool {
    match (literal, target_type) {
        // Integer literal to integer type conversions
        (Literal::Int(value), Type::I8) => *value >= i8::MIN as i64 && *value <= i8::MAX as i64,
        (Literal::Int(value), Type::I16) => *value >= i16::MIN as i64 && *value <= i16::MAX as i64,
        (Literal::Int(value), Type::I32) => *value >= i32::MIN as i64 && *value <= i32::MAX as i64,
        (Literal::Int(value), Type::I64) => true, // Always fits
        
        (Literal::Int(value), Type::U8) => *value >= 0 && *value <= u8::MAX as i64,
        (Literal::Int(value), Type::U16) => *value >= 0 && *value <= u16::MAX as i64,
        (Literal::Int(value), Type::U32) => *value >= 0 && *value <= u32::MAX as i64,
        (Literal::Int(value), Type::U64) => *value >= 0, // Can't check upper bound as i64 < u64::MAX
        
        // Float literal to float type conversions can always be performed (though precision may be lost)
        (Literal::Float(_), Type::FP32) | (Literal::Float(_), Type::FP64) => true,
        
        // Other literal types must match exactly
        (Literal::String(_), Type::String) => true,
        (Literal::Char(_), Type::Char) => true,
        (Literal::Bool(_), Type::Bool) => true,
        
        // All other conversions are invalid
        _ => false,
    }
}

/// Check if a type is any numeric type (integer or floating point)
pub fn is_numeric_type(t: &Type) -> bool {
    matches!(t, 
        Type::U8 | Type::U16 | Type::U32 | Type::U64 |
        Type::I8 | Type::I16 | Type::I32 | Type::I64 |
        Type::FP8 | Type::FP16 | Type::FP32 | Type::FP64
    )
}

/// Check if a type is any integer type (signed or unsigned)
pub fn is_integer_type(t: &Type) -> bool {
    matches!(t, 
        Type::U8 | Type::U16 | Type::U32 | Type::U64 |
        Type::I8 | Type::I16 | Type::I32 | Type::I64
    )
}

/// Return the "wider" of two numeric types
pub fn wider_numeric_type(t1: Type, t2: Type) -> Type {
    match (&t1, &t2) {
        // If either is a floating point, promote to the largest floating point
        (_, Type::FP64) | (Type::FP64, _) => Type::FP64,
        (_, Type::FP32) | (Type::FP32, _) => Type::FP32,
        (_, Type::FP16) | (Type::FP16, _) => Type::FP16,
        (_, Type::FP8) | (Type::FP8, _) => Type::FP8,
        
        // For integers, use the largest signed or unsigned
        (_, Type::I64) | (Type::I64, _) => Type::I64,
        (_, Type::U64) | (Type::U64, _) => Type::U64,
        (_, Type::I32) | (Type::I32, _) => Type::I32,
        (_, Type::U32) | (Type::U32, _) => Type::U32,
        (_, Type::I16) | (Type::I16, _) => Type::I16,
        (_, Type::U16) | (Type::U16, _) => Type::U16,
        (_, Type::I8) | (Type::I8, _) => Type::I8,
        (_, Type::U8) | (Type::U8, _) => Type::U8,
        
        // This shouldn't happen due to the is_numeric_type check earlier
        _ => t1,
    }
}

/// Check if a cast is valid between two types
pub fn is_valid_cast(from: &Type, to: &Type) -> bool {
    match (from, to) {
        // Numeric types can be cast to other numeric types
        (f, t) if is_numeric_type(f) && is_numeric_type(t) => true,
        
        // Char can be cast to/from integer
        (Type::Char, t) if is_integer_type(t) => true,
        (f, Type::Char) if is_integer_type(f) => true,
        
        // Bool can be cast to/from integer
        (Type::Bool, t) if is_integer_type(t) => true,
        (f, Type::Bool) if is_integer_type(f) => true,
        
        // Same types are always castable
        (f, t) if types_match(f, t) => true,
        
        // All other casts are invalid
        _ => false,
    }
}


