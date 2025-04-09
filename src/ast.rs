// NOTE: ENTRY POINT
#[derive(Debug)]
pub struct Program {
    pub definitions: Vec<TopLevelDefinition>,
}

#[derive(Debug)]
pub enum TopLevelDefinition {
    Function(FunctionDef),
    Struct(StructDef),
}

#[derive(Debug)]
pub struct FunctionDef {
    pub return_type: Option<Type>,
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub param_type: Type,
}

#[derive(Debug)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
}


// NOTE: STATEMENTS
#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),
    If(IfStatement),
    While(WhileStatement),
    Break,
    Continue,
    Return(ReturnStatement),
    FunctionCall(FunctionCall),

}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub modifiers: bool,
    pub var_type: Type,
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Assignment {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub else_if_blocks: Vec<ElseIf>,
    pub else_block: Option<Vec<Statement>>,
}

#[derive(Debug)]
pub struct ElseIf {
    pub condition: Expression,
    pub block: Vec<Statement>,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
}

// INFO: EXPRESSIONS

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    BinaryOp(BinaryOperation),
    UnaryOp(UnaryOperation),
    FunctionCall(FunctionCall),
    StructInit(StructInit),
    StructFieldAccess(StructFieldAccess),
    ArrayInit(Box<ArrayInit>),
    ArrayAccess(Box<ArrayAccess>),
    VectorInit(Box<VectorInit>),
    TypeCast(TypeCast),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct UnaryOperation {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug)]
pub struct StructInit {
    pub struct_name: String,
    pub fields: Vec<StructFieldInit>,
}

#[derive(Debug)]
pub struct StructFieldInit {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct StructFieldAccess {
    pub struct_name: String,
    pub field_name: String,
}

#[derive(Debug)]
pub struct ArrayInit {
    pub array_type: Type,
    pub size: usize,
    pub initial_value: Option<Expression>,
}

#[derive(Debug)]
pub struct ArrayAccess {
    pub array_name: String,
    pub index: Expression,
}

#[derive(Debug)]
pub struct VectorInit {
    pub vector_type: Type,
    pub initial_value: Option<Expression>,
}

#[derive(Debug)]
pub struct TypeCast {
    pub expression: Box<Expression>,
    pub target_type: Type,
}

#[derive(Debug)]
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
    Array(Box<Type>, Option<usize>),
    Vector(Box<Type>),
}
