use super::statements::Statement;
use super::types::Type;

// Top level program structure
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
