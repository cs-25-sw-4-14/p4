use crate::{
    ast::{ArrayAccess, ArrayLiteral, Expression, Literal, StructFieldAccess, Type, TypeCast},
    lossy,
};

#[test]
fn test_struct_field_init() {
    assert_eq!(
        Expression::StructFieldAccess(StructFieldAccess {
            struct_expr: Box::new(Expression::Variable("test".to_string())),
            field_name: "field".to_string(),
        }),
        lossy::ExpressionParser::new().parse("test.field").unwrap()
    );
}

#[test]
fn test_struct_field_access() {
    assert_eq!(
        Expression::StructFieldAccess(StructFieldAccess {
            struct_expr: Box::new(Expression::Variable("test".to_string())),
            field_name: "field".to_string(),
        }),
        lossy::ExpressionParser::new().parse("test.field").unwrap()
    );
}

#[test]
fn test_array_access() {
    assert_eq!(
        ArrayAccess {
            array_name: "test".to_string(),
            index: Expression::Literal(Literal::Int(32))
        },
        lossy::ArrayAccessParser::new().parse("test[32]").unwrap()
    );
}

#[test]
fn test_array_literal() {
    assert_eq!(
        ArrayLiteral {
            elements: vec![
                Expression::Literal(Literal::Int(1)),
                Expression::Literal(Literal::Int(2)),
                Expression::Literal(Literal::Int(3)),
            ],
        },
        lossy::ArrayLiteralParser::new().parse("[1,2,3]").unwrap()
    );
}

#[test]
fn test_type_cast() {
    assert_eq!(
        Expression::TypeCast(TypeCast {
            expression: Box::new(Expression::Literal(Literal::Int(32))),
            target_type: Type::U8
        }),
        lossy::ExpressionParser::new().parse("bool").unwrap()
    );

    assert_eq!(
        Expression::TypeCast(TypeCast {
            expression: Box::new(Expression::Variable("test".to_string())),
            target_type: Type::U8
        }),
        lossy::ExpressionParser::new().parse("bool").unwrap()
    );
}

#[test]
fn test_type() {
    assert_eq!(Type::U8, lossy::TypeParser::new().parse("u8").unwrap());
    assert_eq!(Type::U16, lossy::TypeParser::new().parse("u16").unwrap());
    assert_eq!(Type::U32, lossy::TypeParser::new().parse("u32").unwrap());
    assert_eq!(Type::U64, lossy::TypeParser::new().parse("u64").unwrap());
    assert_eq!(Type::I8, lossy::TypeParser::new().parse("i8").unwrap());
    assert_eq!(Type::I16, lossy::TypeParser::new().parse("i16").unwrap());
    assert_eq!(Type::I32, lossy::TypeParser::new().parse("i32").unwrap());
    assert_eq!(Type::I64, lossy::TypeParser::new().parse("i64").unwrap());
    assert_eq!(Type::FP16, lossy::TypeParser::new().parse("fp16").unwrap());
    assert_eq!(Type::FP32, lossy::TypeParser::new().parse("fp32").unwrap());
    assert_eq!(Type::FP64, lossy::TypeParser::new().parse("fp64").unwrap());
    assert_eq!(
        Type::String,
        lossy::TypeParser::new().parse("String").unwrap()
    );
    assert_eq!(Type::Char, lossy::TypeParser::new().parse("char").unwrap());
    assert_eq!(Type::Bool, lossy::TypeParser::new().parse("bool").unwrap());
    assert_eq!(
        Type::Vector(Box::new(Type::U8)),
        lossy::TypeParser::new().parse("Vec<u8>").unwrap()
    );
    assert_eq!(
        Type::Array(Box::new(Type::U8), 32),
        lossy::TypeParser::new().parse("Array u8[32]").unwrap()
    );
}
