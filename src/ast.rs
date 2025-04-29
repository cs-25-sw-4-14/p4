#[cfg(test)]
pub mod integration_tests;
#[cfg(test)]
pub mod unit_tests;

// NOTE: ENTRY POINT
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub definitions: Vec<TopLevelDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelDefinition {
    Function(FunctionDef),
    Struct(StructDef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef {
    pub return_type: Option<Type>,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
}

// NOTE: STATEMENTS
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Throwaway(Throwaway, usize),
    VariableDeclaration(VariableDeclaration, usize),
    Assignment(VariableAssignment, usize),
    ArrayAssignment(ArrayAssignment, usize),
    If(IfStatement, usize),
    While(WhileStatement, usize),
    Break(usize),
    Continue(usize),
    Return(ReturnStatement, usize),
    FunctionCall(FunctionCall, usize),
    Print(Print, usize),
    Write(Write, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Throwaway {
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub trivial: Option<()>,
    pub mutable: Option<()>,
    pub var_type: Type,
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayAssignment {
    pub target: ArrayAccess,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableAssignment {
    pub target: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Vec<Statement>,
    pub else_if_blocks: Vec<ElseIf>,
    pub else_block: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseIf {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Write {
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Read {
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Print {
    pub expression: Expression,
}

// INFO: EXPRESSIONS

#[derive(Debug, Clone, PartialEq)]
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
    Read(Box<Read>),
    Reference(Box<Expression>, bool), // (expression, is_mutable)
    Dereference(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(u64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Multiply,
    Divide,
    Modulus,
    Add,
    Subtract,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructInit {
    pub name: String,
    pub fields: Vec<StructFieldInit>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructFieldInit {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructFieldAccess {
    pub struct_expr: Box<Expression>,
    pub field_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayAccess {
    pub array_name: String,
    pub index: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayLiteral {
    pub elements: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeCast {
    pub expression: Box<Expression>,
    pub target_type: Type,
}

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
    FP16,
    FP32,
    FP64,
    String,
    Char,
    Bool,
    Struct(String),
    Array(Box<Type>, u64),
    Vector(Box<Type>),
    Reference(Box<Type>, bool), // (type, is_mutable)
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
