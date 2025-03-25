grammar lzy;

// Top-level program structure
program
    : (stmt)+ EOF
    ;

// Statements can be variable declarations or expressions
stmt
    : (varDecl| expr) ';'
    ;

// Variable declaration with optional initialization
varDecl
    : type ID ('=' expr)?
    ;

// Expressions with proper precedence
expr
    : assignExpr
    ;

// Assignment expression
assignExpr
    : ID '=' additiveExpr
    | additiveExpr
    ;

// Additive expressions (addition and subtraction)
additiveExpr
    : multExpr ('+' multExpr | '-' multExpr)*
    ;

// Multiplicative expressions (multiplication, division, modulo)
multExpr
    : primaryExpr (('*' | '/' | '%') primaryExpr)*
    ;

// Primary expressions (identifiers, literals, parenthesized expressions)
primaryExpr
    : ID
    | INT
    | '(' expr ')'
    ;

// Type system
type
    : 'u8'    | 'u16'   | 'u32'   | 'u64'
    | 'i8'    | 'i16'   | 'i32'   | 'i64'
    | 'f8'    | 'f16'   | 'f32'   | 'f64'
    | 'bool'  | 'string'
    ;

// Lexer rules
ID: [a-zA-Z][a-zA-Z0-9]*;
INT: '0' | [1-9][0-9]*;
WS: [ \t\r\n]+ -> skip;

// Removed unused rules and simplified the grammar
