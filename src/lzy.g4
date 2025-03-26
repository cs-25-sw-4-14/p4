grammar lzy;

// Top-level program structure
program
    : definition+ EOF
    ;

// Statements can be variable declarations or expressions

definition
    : function
    ;

function
    : (VOID|type) ID LEFTPARENTESIS ((type ID) (COMMA type ID )*)?  RIGHTPARENTESIS LEFTCURLY
    stmt*
    RIGHTCURLY
    ;
functionCall:
    ID LEFTPARENTESIS type ID (COMMA type ID )*  RIGHTPARENTESIS SEMICOLON
    stmt* 
    ;

stmt 
    : (loop
    | definition
    | functionCall
    | conditional
    | assignment) SEMICOLON;

conditional:
;

loop:
;


// Assignment expression
assignment
    : TRIVIAL? MUTABLE? (INTTYPE | UINTTYPE | FLOAT) ID EQUAL additiveExpr
    | TRIVIAL? MUTABLE? STRINGTYPE ID EQUAL textExpr
    | TRIVIAL? MUTABLE? BOOLTYPE ID EQUAL  boolExpr
    | TRIVIAL? MUTABLE? type(ID | UNDERSCORE) EQUAL  functionCall
    ;


// Additive expressions (addition and subtraction)
additiveExpr
    : multExpr (PLUS multExpr | MINUS multExpr)*
    ;

// Multiplicative expressions (multiplication, division, modulo)
multExpr
    : numExpr ((MULTIPLICATION | DIVISION | MODULOS) numExpr)*
    ;

// Primary expressions (identifiers, literals, parenthesized expressions)
numExpr
    : INT
    | FLOAT
    | RIGHTPARENTESIS additiveExpr LEFTPARENTESIS
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
    | RIGHTPARENTESIS boolExpr LEFTPARENTESIS
    ;

type
    : STRINGTYPE
    | UINTTYPE
    | INTTYPE
    | FLOATTYPE
    | BOOLTYPE
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

//id
ID: [a-zA-Z][a-zA-Z0-9]*;
//int
INT: '0' | [1-9][0-9]*;
//float
FLOAT: [0-9]* '.' [0-9]+;
//String
STRING: '"' (~["\r\n])* '"';
//char
CHAR: '\'' ~['\r\n] '\'';
//boolean
BOOL : 'true' | 'false';
//white space
WS: [ \t\r\n]+ -> skip;


TRIVIAL
    : 'triv';
MUTABLE
    : 'mut'; 
VOID
    : 'void';

EQUAL
    : '=';
MULTIPLICATION
    : '*';
MODULOS: '%';
DIVISION
: '/';
UNDERSCORE: '_';
LEFTPARENTESIS: '(';
RIGHTPARENTESIS: ')';
LEFTCURLY: '{';
RIGHTCURLY: '}';
LEFTBRACKET: '[';
 RIGHTBRACKET: ']'; 

PLUS: '+';
MINUS: '-';
COMMA:
','
;
SEMICOLON: ';' ; 

// Removed unused rules and simplified the grammar
