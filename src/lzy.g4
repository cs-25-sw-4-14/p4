// =============================================================================
// lzy Language Grammar Definition
// =============================================================================

grammar lzy;

// =========================== TOP-LEVEL STRUCTURE =============================

// Root rule - a program consists of one or more functions or struct definitions
program
    : (function | structDefinition)+ EOF
    ;

// Function definition - includes return type, name, parameters, and body
function
    : functionType ID LEFTPAREN paramList? RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
    ;

// Function return type - either void or a data type
functionType
    : VOID
    | type
    ;

// Function parameters - list of type-name pairs
paramList
    : type ID (COMMA type ID)*
    ;

// Struct definition - a named collection of fields
structDefinition
    : STRUCT ID LEFTBRACE
      structField*
      RIGHTBRACE
    ;

// Fields within a struct - each has a type and name
structField
    : type ID SEMICOLON
    ;

// ============================== STATEMENTS ==================================

// Statements - the basic units of execution in the language
statement
    : declaration SEMICOLON       // [0] Variable declarations
    | assignment SEMICOLON        // [1] Assignment operations
    | whileLoop                   // [2] While loops
    | ifStatement                 // [3] If-else statements
    | switchStatement             // [4] Switch statements
    | breakStmt SEMICOLON         // [5] Break statements
    | continueStmt SEMICOLON      // [6] Continue statements
    | returnStmt SEMICOLON        // [7] Return statements
    | expr SEMICOLON              // [8] Expression statements
    ;

// Variable declaration - defines a new variable with optional initialization --------------------- [0]
declaration
    : modifier* type ID (EQUAL expr)?
    ;

// Variable modifiers - affects variable behavior (e.g., mutability) ------------------------------ [0]
modifier
    : TRIV    // Trivial/constant
    | MUT     // Mutable
    ;

// Assignment - sets a value to an existing variable or struct field ------------------------------ [1] 
assignment
    : lvalue EQUAL expr
    ;

// Left-value - anything that can appear on the left side of an assignment ------------------------ [1]
lvalue
    : ID              // Variable
    | memberAccess    // Struct field access (e.g., struct.field) [GO TO 10]
    ;

// While loop - executes statements as long as condition is true ---------------------------------- [2]
whileLoop
    : WHILE LEFTPAREN condition RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
    ;

// If statement - conditional execution with optional else-if and else blocks --------------------- [3]
ifStatement
    : IF LEFTPAREN condition RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
      elseIfStatement*
      elseStatement?
    ;

// Else-if statement - additional conditions checked if previous conditions are false ------------- [3]
elseIfStatement
    : ELSE IF LEFTPAREN condition RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
    ;

// Else statement - executed if all previous conditions are false --------------------------------- [3]
elseStatement
    : ELSE LEFTBRACE
      statement*
      RIGHTBRACE
    ;

// ============================== CONDITIONALS (FOR IF STATEMENTS AND WHILE LOOP [2 & 3]) ================================
                                                                                                                      // |
// Condition - used in if statements and loops                                                                        // |
condition                                                                                                             // |
    : logicalExpr                                                                                                     // |
    ;                                                                                                                 // |
                                                                                                                      // |
// Logical expression - combines comparison expressions with AND/OR                                                   // |
logicalExpr                                                                                                           // |
    : comparisonExpr ((AND | OR) comparisonExpr)*                                                                     // |
    ;                                                                                                                 // |
                                                                                                                      // |
// Comparison expression - compares expressions using operators                                                       // |
comparisonExpr                                                                                                        // |
    : expr (comparisonOp expr)?                                                                                       // |
    ;                                                                                                                 // |
                                                                                                                      // |
// Comparison operators                                                                                               // |
comparisonOp                                                                                                          // |
    : EQUAL_EQUAL    // Equal to (==)                                                                                 // |
    | NOT_EQUAL      // Not equal to (!=)                                                                             // |
    | LESS           // Less than (<)                                                                                 // |
    | LESS_EQUAL     // Less than or equal to (<=)                                                                    // |
    | GREATER        // Greater than (>)                                                                              // |
    | GREATER_EQUAL  // Greater than or equal to (>=)                                                                 // |
    ;                                                                                                                 // |
                                                                                                                      // |
// ======================================================= END ========================================================= |

// Switch statement - selects execution path based on expression value ---------------------------- [4]
switchStatement
    : SWITCH LEFTPAREN expr RIGHTPAREN LEFTBRACE
      caseStatement*
      defaultStatement?
      RIGHTBRACE
    ;

// Case statement - executed if switch expression matches case expression ------------------------- [4]
caseStatement
    : CASE expr COLON
      statement*
    ;

// Default statement - executed if no cases match the switch expression --------------------------- [4]
defaultStatement
    : DEFAULT COLON
      statement*
    ;

// Break statement - exits the nearest enclosing loop or switch ----------------------------------- [5]
breakStmt
    : BREAK
    ;

// Continue statement - skips to the next iteration of the nearest enclosing loop ----------------- [6]
continueStmt
    : CONTINUE
    ;

// Return statement - exits a function with optional return value --------------------------------- [7]
returnStmt
    : RETURN expr?
    ;

// ============================== EXPRESSIONS =================================

// Expression - the building block for conditions and values -------------------------------------- [8]
expr
    : conditionalExpr
    ;

// Conditional expression - ternary operator (condition ? true_expr : false_expr) ----------------- [8]
conditionalExpr
    : logicalOrExpr (QUESTION expr COLON expr)?
    ;

// Logical OR expression - combines AND expressions with OR --------------------------------------- [8]
logicalOrExpr
    : logicalAndExpr (OR logicalAndExpr)*
    ;

// Logical AND expression - combines equality expressions with AND -------------------------------- [8]
logicalAndExpr
    : equalityExpr (AND equalityExpr)*
    ;

// Equality expression - compares expressions for equality/inequality ----------------------------- [8]
equalityExpr
    : relationalExpr ((EQUAL_EQUAL | NOT_EQUAL) relationalExpr)*
    ;

// Relational expression - compares expressions with ordering operators --------------------------- [8]
relationalExpr
    : additiveExpr ((LESS | LESS_EQUAL | GREATER | GREATER_EQUAL) additiveExpr)*
    ;

// Additive expression - performs addition and subtraction ---------------------------------------- [8]
additiveExpr
    : multiplicativeExpr ((PLUS | MINUS) multiplicativeExpr)*
    ;

// Multiplicative expression - performs multiplication, division, and modulo ---------------------- [8]
multiplicativeExpr
    : unaryExpr ((MUL | DIV | MOD) unaryExpr)*
    ;

// Unary expression - applies unary operators to expressions -------------------------------------- [8]
unaryExpr
    : (MINUS | NOT) unaryExpr  // Negation or logical NOT
    | primaryExpr              // Base expressions
    ;

// Primary expression - the most basic expression types ------------------------------------------- [8]
primaryExpr
    : ID                         // Variable reference
    | INT                        // Integer literal
    | FLOAT                      // Floating-point literal
    | STRING_LITERAL             // String literal
    | BOOL_LITERAL               // Boolean literal
    | structInit                 // [9] Struct initialization
    | memberAccess               // [10] Struct field access
    | LEFTPAREN expr RIGHTPAREN  // Parenthesized expression
    ;

// Struct initialization - creates a new struct instance ------------------------------------------ [9]
structInit
    : ID LEFTBRACE structInitField (COMMA structInitField)* RIGHTBRACE
    ;


// Struct initialization field - sets an initial value for a struct field ------------------------- [9]
structInitField
    : ID COLON expr
    ;

// Member access - retrieves a field from a struct ------------------------------------------------ [10]
memberAccess
    : ID DOT ID (DOT ID)*  // Supports nested access (a.b.c)
    ;

// ============================== TYPE SYSTEM ================================

// Type - all possible data types in the language
type
    : STRING      // String type
    | UINT        // Unsigned integer types (u8, u16, etc.)
    | INT_TYPE    // Signed integer types (i8, i16, etc.)
    | FLOAT_TYPE  // Floating-point types (f8, f16, etc.)
    | BOOL        // Boolean type
    | ID          // User-defined struct types
    ;

// ============================== LEXER RULES ================================

// -------------------- KEYWORDS --------------------

// Control flow and declaration keywords
VOID    : 'void' ;
TRIV    : 'triv' ;
MUT     : 'mut'  ;
WHILE   : 'while' ;
IF      : 'if' ;
ELSE    : 'else' ;
SWITCH  : 'switch' ;
CASE    : 'case' ;
DEFAULT : 'default' ;
BREAK   : 'break' ;
CONTINUE: 'continue' ;
RETURN  : 'return' ;
STRUCT  : 'struct' ;

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
QUESTION       : '?' ;   // Ternary operator (question part)
COLON          : ':' ;   // Ternary operator (colon part) or case label
DOT            : '.' ;   // Member access

// -------------------- PUNCTUATION --------------------

// Brackets and braces
LEFTPAREN      : '(' ;   // Opening parenthesis
RIGHTPAREN     : ')' ;   // Closing parenthesis
LEFTBRACE      : '{' ;   // Opening curly brace
RIGHTBRACE     : '}' ;   // Closing curly brace

// Other punctuation
COMMA          : ',' ;   // Comma for separating items
SEMICOLON      : ';' ;   // Semicolon for ending statements

// -------------------- LITERALS --------------------

// Identifiers (variable and function names)
ID    : [a-zA-Z][a-zA-Z0-9_]* ;  // Starts with a letter, followed by letters, digits, or underscores

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
// COMMENT : '//' ~[\r\n]* -> skip ;
COMMENT : '<!--' .*? '-->' -> skip ; // Obviously the superior comment style - Marc the dork 💪😎
