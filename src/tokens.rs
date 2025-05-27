use logos::Logos;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use unescape::unescape;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    InvalidFloat(ParseFloatError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

impl From<ParseFloatError> for LexicalError {
    fn from(err: ParseFloatError) -> Self {
        LexicalError::InvalidFloat(err)
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalError::InvalidInteger(e) => write!(f, "invalid integer: {}", e),
            LexicalError::InvalidFloat(e) => write!(f, "invalid float: {}", e),
            LexicalError::InvalidToken => write!(f, "invalid token"),
        }
    }
}
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+", skip r"//[^\n\r]*", skip r"/\*([^*]|\*+([^*/]|\*+))*\*+\*/", error = LexicalError)]
pub enum Token {
    #[token("mut")]
    KeywordMut,
    #[token("triv")]
    KeywordTrivial,
    #[token("&mut")]
    KeywordRefMut,
    #[token("&")]
    KeywordRef,
    #[token("arr")]
    TypeArray,
    #[token("Vec")]
    TypeVector,
    #[token("bool")]
    TypeBool,
    #[token("char")]
    TypeChar,
    #[token("String")]
    TypeString,
    #[token("f64")]
    TypeF64,
    #[token("f32")]
    TypeF32,
    #[token("f16")]
    TypeF16,
    #[token("i64")]
    TypeI64,
    #[token("i32")]
    TypeI32,
    #[token("i16")]
    TypeI16,
    #[token("i8")]
    TypeI8,
    #[token("u64")]
    TypeU64,
    #[token("u32")]
    TypeU32,
    #[token("u16")]
    TypeU16,
    #[token("u8")]
    TypeU8,
    #[token("null")]
    KeywordNull,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("||")]
    OperatorOr,
    #[token("&&")]
    OperatorAnd,
    #[token("!=")]
    OperatorNotEqual,
    #[token("==")]
    OperatorEqual,
    #[token(">=")]
    OperatorGreaterEqual,
    #[token("<=")]
    OperatorLessEqual,
    #[token(">")]
    OperatorGreater,
    #[token("<")]
    OperatorLess,
    #[token("+")]
    OperatorAdd,
    #[token("%")]
    OperatorMod,
    #[token("*")]
    Star,
    #[token("/")]
    OperatorDiv,
    #[token("!")]
    OperatorNot,
    #[token("-")]
    OperatorNeg,
    #[token("as")]
    OperatorAs,
    #[token("read_file")]
    KeywordRead,
    #[token("write_file")]
    KeywordWrite,
    #[token(".insert")]
    MethodInsert,
    #[token(".remove")]
    MethodRemove,
    #[token(".get")]
    MethodGet,
    #[token("debug")]
    KeywordDebug,
    #[token("return")]
    KeywordReturn,
    #[token("while")]
    KeywordWhile,
    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("=")]
    Assign,
    #[token("struct")]
    KeywordStruct,
    #[token("void")]
    KeywordVoid,
    #[token("break")]
    KeywordBreak,
    #[token("continue")]
    KeywordContinue,

    // identifiers
    #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().to_string())]
    Identifier(String),

    // integer literals (decimal)
    #[regex("0|[1-9][0-9]*", |lex| lex.slice().parse::<u64>(), priority = 1)]
    IntLiteral(u64),

    // float literals (must have a decimal point)
    #[regex("[0-9]+\\.[0-9]+", |lex| lex.slice().parse::<f64>())]
    FloatLiteral(f64),

    // character literals, e.g. 'a' or '\n'
    #[regex("'([^'\\\\\r\n]|\\\\.)'", |lex| {
        let s = lex.slice();
        // unwrap single char between the quotes
        let mut chars = s[1..s.len()-1].chars();
        chars.next().ok_or(LexicalError::InvalidToken)
    })]
    CharLiteral(char),

    #[regex(r#""(?:[^"\\]|\\.)*""#, |lex| {
    let s = &lex.slice()[1..lex.slice().len() - 1]; // Strip quotes
    unescape(s).unwrap_or_else(|| s.to_string())   // Unescape \n -> newline
    })]
    StringLiteral(String),

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    BoolLiteral(bool),
}
