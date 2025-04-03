grammar parserExpressions;
options  {tokenVocab=lzyLexer;}

// ============================== EXPRESSIONS =================================

// Expression - the building block for conditions and values -------------------------------------- [7]
// Example --- a + b * c
expr
    : logicalOrExpr
    ;

// Logical OR expression - combines AND expressions with OR --------------------------------------- [7]
// Example --- isAdmin || hasSpecialPermission
logicalOrExpr
    : logicalAndExpr (OR logicalAndExpr)*
    ;

// Logical AND expression - combines equality expressions with AND -------------------------------- [7]
// Example --- isLoggedIn && hasPermission
logicalAndExpr
    : equalityExpr (AND equalityExpr)*
    ;

// Equality expression - compares expressions for equality/inequality ----------------------------- [7]
// Example --- username == "admin"
// Example --- status != "error"
equalityExpr
    : relationalExpr ((EQUAL_EQUAL | NOT_EQUAL) relationalExpr)*
    ;

// Relational expression - compares expressions with ordering operators --------------------------- [7]
// Example --- age > 18
// Example --- score <= 100
relationalExpr
    : additiveExpr ((LESS | LESS_EQUAL | GREATER | GREATER_EQUAL) additiveExpr)*
    ;

// Additive expression - performs addition and subtraction ---------------------------------------- [7]
// Example --- x + y
// Example --- total - discount
additiveExpr
    : multiplicativeExpr ((PLUS | MINUS) multiplicativeExpr)*
    ;

// Multiplicative expression - performs multiplication, division, and modulo ---------------------- [7]
// Example --- width * height
// Example --- total / count
// Example --- value % 10
multiplicativeExpr
    : unaryExpr ((MUL | DIV | MOD) unaryExpr)*
    ;

// Unary expression - applies unary operators to expressions -------------------------------------- [7]
// Example --- -x
// Example --- !isValid
unaryExpr
    : (MINUS | NOT) unaryExpr  // Negation or logical NOT
    | primaryExpr              // Base expressions
    ;

// Primary expression - the most basic expression types ------------------------------------------- [7]
// Example --- x (variable)
// Example --- 69420 (integer literal)
// Example --- "hello" (string literal)
// Example --- Person {name: "Marc", age: 24} (struct initialization)
// Example --- user.name (member access)
// Example --- a[0] (array access)
// Example --- (x + y) (parenthesized expression)
primaryExpr
    : ID                         // Variable reference
    | INT                        // Integer literal
    | FLOAT                      // Floating-point literal
    | STRING_LITERAL             // String literal
    | BOOL_LITERAL               // Boolean literal
    | structInit                 // [8] Struct initialization
    | arrayInit                  // Array initialization
    | vectorInit                 // Vector initialization
    | memberAccess               // [9] Struct field access
    | arrayAccess                // Array/Vector element access
    | functionCall               // Function call
    | methodCall                 // Method call for vectors and strings
    | methodAccess               // Method access with chaining support
    | LEFTPAREN expr RIGHTPAREN  // Parenthesized expression
    ;
