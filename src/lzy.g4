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
    : TYPE ID ('=' expr)? |
    TYPE ID
    ;

// Expressions with proper precedence
expr
    : assignExpr
    ;

// Assignment expression
assignExpr
    : ID '=' additiveExpr
    | ID '=' textExpr
    | ID '=' boolExpr
    | additiveExpr
    | textExpr
    | boolExpr
    ;

// Additive expressions (addition and subtraction)
additiveExpr
    : multExpr ('+' multExpr | '-' multExpr)*
    ;

// Multiplicative expressions (multiplication, division, modulo)
multExpr
    : numExpr (('*' | '/' | '%') numExpr)*
    ;

// Primary expressions (identifiers, literals, parenthesized expressions)
numExpr
    : INT
    | FLOAT
    | '(' expr ')'
    | ID
    ;
textExpr
    : STRING
    | CHAR
    | ID
    ;
boolExpr
    : BOOL
    | ID
    ;



// Type system
TYPE
    : 'u8'    | 'u16'   | 'u32'   | 'u64'
    | 'i8'    | 'i16'   | 'i32'   | 'i64'
    | 'f8'    | 'f16'   | 'f32'   | 'f64'
    | 'bool'  | 'string'
    ;

ID: [a-zA-Z][a-zA-Z0-9]*;
INT: '0' | [1-9][0-9]*;
FLOAT: [0-9]* '.' [0-9]+;
STRING: '"' (~["\r\n])* '"';
CHAR: '\'' ~['\r\n] '\'';
WS: [ \t\r\n]+ -> skip;
BOOL : 'true' | 'false';

// Removed unused rules and simplified the grammar
