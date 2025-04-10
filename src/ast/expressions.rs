use super::types::Type;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    BinaryOp(BinaryOperation),
    UnaryOp(UnaryOperation),
    FunctionCall(FunctionCall),
    StructInit(StructInit),
    StructFieldAccess(StructFieldAccess),
    ArrayAccess(Box<ArrayAccess>),
    ArrayLiteral(Box<ArrayLiteral>),
    TypeCast(TypeCast),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Multiply,       // 1
    Divide,         // 1
    Modulus,        // 1
    Add,            // 2
    Subtract,       // 2
    LessThan,       // 3
    GreaterThan,    // 3
    LessThanOrEqual,// 3
    GreaterThanOrEqual, // 3
    Equal,          // 4
    NotEqual,       // 4
    And,            // 5
    Or,             // 6
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub struct StructInit {
    pub name: String,
    pub fields: Vec<StructFieldInit>,
}

#[derive(Debug, Clone)]
pub struct StructFieldInit {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct StructFieldAccess {
    pub struct_expr: Box<Expression>,
    pub field_name: String,
}

#[derive(Debug, Clone)]
pub struct ArrayAccess {
    pub array_name: String,
    pub index: Expression,
}

#[derive(Debug, Clone)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct TypeCast {
    pub expression: Box<Expression>,
    pub target_type: Type,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
}
