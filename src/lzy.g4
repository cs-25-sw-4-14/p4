grammar lzy;

// Top-level program structure
program
    : (stmt)+ EOF
    ;

// Statements can be variable declarations or expressions
stmt
    : (expr) ';'
    ;



// Expressions with proper precedence
expr
    : assignExpr
    ;

// Assignment expression
assignExpr
    : 'triv'? 'mut'? (INTTYPE | UINTTYPE | FLOAT) ID '=' additiveExpr
    | 'triv'? 'mut'? STRINGTYPE '=' textExpr
    | 'triv'? 'mut'? BOOLTYPE ID '=' boolExpr
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


STRINGTYPE
    : 'String'
    ;
UINTTYPE
    : 'u8'    | 'u16'   | 'u32'   | 'u64'
    ;
INTTYPE
    : 'i8'    | 'i16'   | 'i32'   | 'i64'
    ;
FLOATTYPE
    : 'f8'    | 'f16'   | 'f32'   | 'f64'
    ;
BOOLTYPE
    : 'bool'
    ;

ID: [a-zA-Z][a-zA-Z0-9]*;
INT: '0' | [1-9][0-9]*;
FLOAT: [0-9]* '.' [0-9]+;
STRING: '"' (~["\r\n])* '"';
CHAR: '\'' ~['\r\n] '\'';
WS: [ \t\r\n]+ -> skip;
BOOL : 'true' | 'false';

// Removed unused rules and simplified the grammar
