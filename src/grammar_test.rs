use crate::{
    ast::{
        ArrayAccess, ArrayLiteral, BinaryOperation, BinaryOperator, ElseIf, Expression,
        FunctionCall, FunctionDef, IfStatement, Literal, Param, Program, ReturnStatement,
        Statement, StructDef, StructField, StructFieldAccess, StructFieldInit, StructInit,
        TopLevelDefinition, Type, TypeCast, UnaryOperation, UnaryOperator, VariableAssignment,
        VariableDeclaration, WhileStatement,
    },
    lossy::{self, FunctionDefParser, StatementParser, StructDefParser},
};

#[test]
fn test_top_level() {
    assert_eq!(
        Program {
            definitions: vec![
                TopLevelDefinition::Struct(StructDef {
                    name: "test".to_string(),
                    fields: vec![
                        StructField {
                            name: "field".to_string(),
                            field_type: Type::U8,
                        },
                        StructField {
                            name: "field2".to_string(),
                            field_type: Type::U16,
                        },
                    ],
                }),
                TopLevelDefinition::Function(FunctionDef {
                    return_type: Some(Type::U32),
                    name: "test".to_string(),
                    params: vec![
                        Param {
                            name: "arg1".to_string(),
                            param_type: Type::U8,
                        },
                        Param {
                            name: "arg2".to_string(),
                            param_type: Type::U16,
                        },
                    ],
                    body: vec![],
                }),
            ],
        },
        lossy::ProgramParser::new()
            .parse("struct test { u8 field; u16 field2; } u32 test(u8 arg1, u16 arg2) {}")
            .unwrap()
    );
}

#[test]
fn function_definition() {
    assert_eq!(
        FunctionDef {
            return_type: Some(Type::U32),
            name: "test".to_string(),
            params: vec![
                Param {
                    name: "arg1".to_string(),
                    param_type: Type::U8,
                },
                Param {
                    name: "arg2".to_string(),
                    param_type: Type::U16,
                },
            ],
            body: vec![],
        },
        FunctionDefParser::new()
            .parse("u32 test(u8 arg1, u16 arg2) {}")
            .unwrap()
    );
}

#[test]
fn test_struct_definition() {
    assert_eq!(
        StructDef {
            name: "test".to_string(),
            fields: vec![
                StructField {
                    name: "field".to_string(),
                    field_type: Type::U8,
                },
                StructField {
                    name: "field2".to_string(),
                    field_type: Type::U16,
                },
            ],
        },
        StructDefParser::new()
            .parse("struct test { u8 field; u16 field2; }")
            .unwrap()
    );
}

#[test]
fn test_variable_assignment() {
    assert_eq!(
        Statement::Assignment(VariableAssignment {
            target: "test".to_string(),
            value: Expression::Literal(Literal::Int(32)),
        }),
        StatementParser::new().parse("test = 32;").unwrap()
    );
}

#[test]
fn test_variable_declaration() {
    assert_eq!(
        Statement::VariableDeclaration(VariableDeclaration {
            trivial: None,
            mutable: None,
            var_type: Type::U8,
            name: "test".to_string(),
            value: Expression::Literal(Literal::Int(32)),
        }),
        StatementParser::new().parse("u8 test = 32;").unwrap()
    );
}

#[test]
fn test_mut_variable_declaration() {
    assert_eq!(
        Statement::VariableDeclaration(VariableDeclaration {
            trivial: Some(()),
            mutable: Some(()),
            var_type: Type::U8,
            name: "test".to_string(),
            value: Expression::Literal(Literal::Int(32)),
        }),
        StatementParser::new()
            .parse("triv mut u8 test = 32;")
            .unwrap()
    );
}

#[test]
fn test_if_statement() {
    assert_eq!(
        Statement::If(IfStatement {
            condition: Expression::BinaryOp(BinaryOperation {
                operator: BinaryOperator::GreaterThan,
                left: Box::new(Expression::Literal(Literal::Int(2))),
                right: Box::new(Expression::Literal(Literal::Int(3))),
            }),
            then_block: vec![],
            else_if_blocks: vec![ElseIf {
                condition: Expression::BinaryOp(BinaryOperation {
                    operator: BinaryOperator::LessThan,
                    left: Box::new(Expression::Literal(Literal::Int(2))),
                    right: Box::new(Expression::Literal(Literal::Int(3))),
                }),
                body: vec![],
            },],
            else_block: Some(vec![]),
        }),
        StatementParser::new()
            .parse("if (2 > 3) {} else if (2 < 3) {} else {}")
            .unwrap()
    );
}

#[test]
fn test_while_statement() {
    assert_eq!(
        Statement::While(WhileStatement {
            condition: Expression::Literal(Literal::Bool(true)),
            body: vec![],
        }),
        StatementParser::new().parse("while (true) {}").unwrap()
    );
}

#[test]
fn test_break_statement() {
    assert_eq!(
        Statement::Break,
        StatementParser::new().parse("break;").unwrap()
    );
}

#[test]
fn test_continue_statement() {
    assert_eq!(
        Statement::Continue,
        StatementParser::new().parse("continue;").unwrap()
    );
}

#[test]
fn test_return_statement() {
    assert_eq!(
        Statement::Return(ReturnStatement {
            value: Some(Expression::Literal(Literal::Int(32))),
        }),
        StatementParser::new().parse("return 32;").unwrap()
    );
}

#[test]
fn test_empty_return_statement() {
    assert_eq!(
        Statement::Return(ReturnStatement { value: None }),
        StatementParser::new().parse("return;").unwrap()
    );
}

#[test]
fn test_literal() {
    assert_eq!(
        Expression::Literal(Literal::Int(32)),
        lossy::ExpressionParser::new().parse("32").unwrap()
    );
    assert_eq!(
        Expression::Literal(Literal::Float(32.0)),
        lossy::ExpressionParser::new().parse("32.0").unwrap()
    );
    assert_eq!(
        Expression::Literal(Literal::String("test".to_string())),
        lossy::ExpressionParser::new().parse("\"test\"").unwrap()
    );
    assert_eq!(
        Expression::Literal(Literal::Char('a')),
        lossy::ExpressionParser::new().parse("'a'").unwrap()
    );
    assert_eq!(
        Expression::Literal(Literal::Bool(true)),
        lossy::ExpressionParser::new().parse("true").unwrap()
    );
}

#[test]
fn test_variable() {
    assert_eq!(
        Expression::Variable("test".to_string()),
        lossy::ExpressionParser::new().parse("test").unwrap()
    );
}

#[test]
fn test_function_call() {
    assert_eq!(
        Expression::FunctionCall(FunctionCall {
            name: "test".to_string(),
            args: vec![Expression::Literal(Literal::Int(32))],
        }),
        lossy::ExpressionParser::new().parse("test(32)").unwrap()
    );
}

#[test]
fn test_function_call_with_multiple_args() {
    assert_eq!(
        Expression::FunctionCall(FunctionCall {
            name: "test".to_string(),
            args: vec![
                Expression::Literal(Literal::Int(32)),
                Expression::Literal(Literal::Int(32)),
            ],
        }),
        lossy::ExpressionParser::new()
            .parse("test(32, 32)")
            .unwrap()
    );
}

#[test]
fn test_function_call_with_no_args() {
    assert_eq!(
        Expression::FunctionCall(FunctionCall {
            name: "test".to_string(),
            args: vec![],
        }),
        lossy::ExpressionParser::new().parse("test()").unwrap()
    );
}

#[test]
fn test_binary_operation() {
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::Divide,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 / 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::Multiply,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 * 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::Modulus,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 % 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::Add,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 + 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::Subtract,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 - 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::LessThan,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 < 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::GreaterThan,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 > 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::LessThanOrEqual,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 <= 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::GreaterThanOrEqual,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 >= 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::Equal,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 == 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::NotEqual,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 != 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::And,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 && 32").unwrap()
    );
    assert_eq!(
        Expression::BinaryOp(BinaryOperation {
            operator: BinaryOperator::Or,
            left: Box::new(Expression::Literal(Literal::Int(32))),
            right: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("32 || 32").unwrap()
    );
}

#[test]
fn test_unary_operation() {
    assert_eq!(
        Expression::UnaryOp(UnaryOperation {
            operator: UnaryOperator::Negate,
            operand: Box::new(Expression::Literal(Literal::Int(32))),
        }),
        lossy::ExpressionParser::new().parse("-32").unwrap()
    );
    assert_eq!(
        Expression::UnaryOp(UnaryOperation {
            operator: UnaryOperator::Not,
            operand: Box::new(Expression::Literal(Literal::Bool(false))),
        }),
        lossy::ExpressionParser::new().parse("!false").unwrap()
    );
}

#[test]
fn test_struct_init() {
    assert_eq!(
        Expression::StructInit(StructInit {
            name: "test".to_string(),
            fields: vec![StructFieldInit {
                name: "field".to_string(),
                value: Expression::Literal(Literal::Int(32)),
            }],
        }),
        lossy::ExpressionParser::new()
            .parse("test { field: 32 }")
            .unwrap()
    );
}

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
        lossy::ExpressionParser::new().parse("32 as u8").unwrap()
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
