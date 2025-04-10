use super::expressions::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    FP8,
    FP16,
    FP32,
    FP64,
    String,
    Char,
    Bool,
    Custom(String),
    Array(Box<Type>, i64),
    Vector(Box<Type>),
}

impl From<Literal> for Type {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Int(_) => Type::I64,
            Literal::Float(_) => Type::FP64,
            Literal::String(_) => Type::String,
            Literal::Char(_) => Type::Char,
            Literal::Bool(_) => Type::Bool,
        }
    }
}
