// NOTE: ENTRY POINT
#[derive(Debug, Clone)]
pub struct Program {
    pub definitions: Vec<TopLevelDefinition>,
}

#[derive(Debug, Clone)]
pub enum TopLevelDefinition {
    Function(FunctionDef),
    Struct(StructDef),
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub return_type: Option<Type>,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
}


// NOTE: STATEMENTS
#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Assignment(VariableAssignment),
    If(IfStatement),
    While(WhileStatement),
    Break,
    Continue,
    Return(ReturnStatement),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub trivial: Option<()>,
    pub mutable: Option<()>,
    pub var_type: Type,
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct VariableAssignment {
    pub target: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Vec<Statement>,
    pub else_if_blocks: Vec<ElseIf>,
    pub else_block: Option<Vec<Statement>>,
}

#[derive(Debug, Clone)]
pub struct ElseIf {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
}

// INFO: EXPRESSIONS

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
    Int(u64),
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    Struct(String),
    Array(Box<Type>, u64),
    Vector(Box<Type>),
}

impl From<Literal> for Type {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::Int(_) => Type::U64,
            Literal::Float(_) => Type::FP64,
            Literal::String(_) => Type::String,
            Literal::Char(_) => Type::Char,
            Literal::Bool(_) => Type::Bool,
        }
    }
}
