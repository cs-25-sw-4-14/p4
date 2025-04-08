use logos::Logos;
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // -------------------- KEYWORDS --------------------
    #[token("void")] Void,
    #[token("triv")] Triv,
    #[token("mut")] Mut,
    #[token("while")] While,
    #[token("if")] If,
    #[token("else")] Else,
    #[token("break")] Break,
    #[token("continue")] Continue,
    #[token("return")] Return,
    #[token("struct")] Struct,
    #[token("Vec")] Vec,
    #[token("char")] Char,

    // -------------------- TYPE LITERALS --------------------
    #[token("String")] StringType,
    #[regex("u(8|16|32|64)")] Uint,
    #[regex("i(8|16|32|64)")] IntType,
    #[regex("f(8|16|32|64)")] FloatType,
    #[token("bool")] BoolType,

    // -------------------- OPERATORS --------------------
    #[token("&&")] And,
    #[token("||")] Or,
    #[token("!")] Not,

    #[token("=")] Equal,
    #[token("==")] EqualEqual,
    #[token("!=")] NotEqual,
    #[token("<")] Less,
    #[token("<=")] LessEqual,
    #[token(">")] Greater,
    #[token(">=")] GreaterEqual,

    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Mul,
    #[token("/")] Div,
    #[token("%")] Mod,

    #[token(":")] Colon,
    #[token(".")] Dot,

    // -------------------- PUNCTUATION --------------------
    #[token("(")] LeftParen,
    #[token(")")] RightParen,
    #[token("{")] LeftBrace,
    #[token("}")] RightBrace,
    #[token("[")] LeftBracket,
    #[token("]")] RightBracket,
    #[token(",")] Comma,
    #[token(";")] Semicolon,

    // -------------------- LITERALS --------------------
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 0)] Id,
    #[regex(r"0|[1-9][0-9]*")] IntLiteral,
    #[regex(r"[0-9]+\.[0-9]+")] FloatLiteral,
    #[regex(r#""([^"\\\r\n]|\\.)*""#)] StringLiteral,
    #[regex(r"true|false")] BoolLiteral,
    #[regex(r"'([^'\\\r\n]|\\.)'")] CharLiteral,

    // -------------------- IGNORED --------------------
    #[regex(r"[ \t\r\n]+", logos::skip)]
    #[regex(r"//[^\r\n]*", logos::skip)]
    CommentOrWs,
}
