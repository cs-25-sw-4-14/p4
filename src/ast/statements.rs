use super::expressions::Expression;
use super::expressions::FunctionCall;
use super::types::Type;

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
