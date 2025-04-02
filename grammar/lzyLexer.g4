lexer grammar lzyLexer;

// ============================== LEXER RULES ================================

// -------------------- KEYWORDS --------------------

// Control flow and declaration keywords
VOID    : 'void' ;
TRIV    : 'triv' ;
MUT     : 'mut'  ;
WHILE   : 'while' ;
IF      : 'if' ;
ELSE    : 'else' ;
BREAK   : 'break' ;
CONTINUE: 'continue' ;
RETURN  : 'return' ;
STRUCT  : 'struct' ;
VEC     : 'Vec' ;

// -------------------- TYPE LITERALS --------------------

// Built-in type names
STRING    : 'String' ;
UINT      : 'u8' | 'u16' | 'u32' | 'u64' ;        // Unsigned integers of various sizes
INT_TYPE  : 'i8' | 'i16' | 'i32' | 'i64' ;        // Signed integers of various sizes
FLOAT_TYPE: 'f8' | 'f16' | 'f32' | 'f64' ;        // Floating-point numbers of various sizes
BOOL      : 'bool' ;

// -------------------- OPERATORS --------------------

// Logical operators
AND            : '&&' ;  // Logical AND
OR             : '||' ;  // Logical OR
NOT            : '!' ;   // Logical NOT

// Assignment and comparison operators
EQUAL          : '=' ;   // Assignment
EQUAL_EQUAL    : '==' ;  // Equal to
NOT_EQUAL      : '!=' ;  // Not equal to
LESS           : '<' ;   // Less than
LESS_EQUAL     : '<=' ;  // Less than or equal to
GREATER        : '>' ;   // Greater than
GREATER_EQUAL  : '>=' ;  // Greater than or equal to

// Arithmetic operators
PLUS           : '+' ;   // Addition
MINUS          : '-' ;   // Subtraction or negation
MUL            : '*' ;   // Multiplication
DIV            : '/' ;   // Division
MOD            : '%' ;   // Modulo (remainder)

// Other operators
COLON          : ':' ;   // Struct members
DOT            : '.' ;   // Member access

// -------------------- PUNCTUATION --------------------

// Brackets and braces
LEFTPAREN      : '(' ;   // Opening parenthesis
RIGHTPAREN     : ')' ;   // Closing parenthesis
LEFTBRACE      : '{' ;   // Opening curly brace
RIGHTBRACE     : '}' ;   // Closing curly brace
LEFTBRACKET    : '[' ;   // Opening square bracket
RIGHTBRACKET   : ']' ;   // Closing square bracket

// Other punctuation
COMMA          : ',' ;   // Comma for separating items
SEMICOLON      : ';' ;   // Semicolon for ending statements

// -------------------- LITERALS --------------------

// Identifiers (variable and function names)
ID    : [a-zA-Z_][a-zA-Z0-9_]* ;  // Starts with a letter, followed by letters, digits, or underscores

// Number literals
INT   : '0' | [1-9][0-9]* ;     // Integer (0 or non-zero followed by digits)
FLOAT : [0-9]+ '.' [0-9]+ ;     // Floating point (digits, decimal point, more digits)

// String literals with escape sequence support
STRING_LITERAL : '"' (~["\\\r\n] | '\\' .)* '"' ;

// Boolean literals
BOOL_LITERAL : 'true' | 'false' ;

// -------------------- IGNORED TOKENS --------------------

// Whitespace - ignored in the parse
WS      : [ \t\r\n]+ -> skip ;

// Comments
COMMENT : '//' ~[\r\n]* -> skip ;

