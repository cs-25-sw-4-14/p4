// =============================================================================
// lzy Language Grammar Definition
// =============================================================================

grammar lzy;

options {tokenVocab=lzyLexer;}

// =========================== TOP-LEVEL STRUCTURE =============================

// Root rule - a program consists of one or more functions or struct definitions
// Example --- struct Person { String name; u32 age; } void main() { ... }
program
    : (function | structDefinition)+ EOF
    ;

// Function definition - includes return type, name, parameters, and body
// Example --- i32 add(i32 a, i32 b) { return a + b; }
function
    : functionType ID LEFTPAREN paramList? RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
    ;

// Function return type - either void or a data type
// Example --- void (for a function that doesn't return a value)
// Example --- i32 (for a function that returns an integer)
functionType
    : VOID
    | type
    ;

// Function parameters - list of type-name pairs
// Example --- u32 age, String name, bool is_dork
paramList
    : type ID (COMMA type ID)*
    ;

// Struct definition - a named collection of fields
// Example --- struct Person { String name; u32 age; bool is_dork; }
structDefinition
    : STRUCT ID LEFTBRACE
      structField*
      RIGHTBRACE
    ;

// Fields within a struct - each has a type and name
// Example --- String name;
// Example --- u32 count;
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
    | breakStmt SEMICOLON         // [4] Break statements
    | continueStmt SEMICOLON      // [5] Continue statements
    | returnStmt SEMICOLON        // [6] Return statements
    | expr SEMICOLON              // [7] Expression statements
    ;

// Variable declaration - defines a new variable with optional initialization --------------------- [0]
// Example --- mut String name = "Marc";
// Example --- triv u32 age = 24;
declaration
    : modifier* type ID (EQUAL expr)?
    ;

// Variable modifiers - affects variable behavior (e.g., mutability) ------------------------------ [0]
// Example --- mut (for mutable variables)
// Example --- triv (for constants/immutable variables)
modifier
    : TRIV    // Trivial/constant
    | MUT     // Mutable
    ;

// Assignment - sets a value to an existing variable or struct field ------------------------------ [1] 
// Example --- count = 10;
// Example --- person.age = 30;
assignment
    : lvalue EQUAL expr
    ;

// Left-value - anything that can appear on the left side of an assignment ------------------------ [1]
// Example --- x (variable)
// Example --- person.name (struct field)
lvalue
    : ID              // Variable
    | memberAccess    // Struct field access (e.g., struct.field) [GO TO 10]
    ;

// While loop - executes statements as long as condition is true ---------------------------------- [2]
// Example --- while (i < 10) { i = i + 1; sum = sum + i; }
whileLoop
    : WHILE LEFTPAREN condition RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
    ;

// If statement - conditional execution with optional else-if and else blocks --------------------- [3]
// Example --- if (x > 10) { return "high"; } else if (x > 5) { return "medium"; } else { return "low"; }
ifStatement
    : IF LEFTPAREN condition RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
      elseIfStatement*
      elseStatement?
    ;

// Else-if statement - additional conditions checked if previous conditions are false ------------- [3]
// Example --- else if (x > 5) { return "medium"; }
elseIfStatement
    : ELSE IF LEFTPAREN condition RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
    ;

// Else statement - executed if all previous conditions are false --------------------------------- [3]
// Example --- else { return "low"; }
elseStatement
    : ELSE LEFTBRACE
      statement*
      RIGHTBRACE
    ;

// ============================== CONDITIONALS (FOR IF STATEMENTS AND WHILE LOOP [2 & 3]) ================================
                                                                                                                      // |
// Condition - used in if statements and loops                                                                        // |
// Example --- x > 0 && y < 10                                                                                        // |
condition                                                                                                             // |
    : logicalExpr                                                                                                     // |
    ;                                                                                                                 // |
                                                                                                                      // |
// Logical expression - combines comparison expressions with AND/OR                                                   // |
// Example --- isValid && count > 3                                                                                   // |
// Example --- isEmpty || hasError                                                                                    // |
logicalExpr                                                                                                           // |
    : comparisonExpr ((AND | OR) comparisonExpr)*                                                                     // |
    ;                                                                                                                 // |
                                                                                                                      // |
// Comparison expression - compares expressions using operators                                                       // |
// Example --- age >= 18                                                                                              // |
// Example --- name == "Admin"                                                                                        // |
comparisonExpr                                                                                                        // |
    : expr (comparisonOp expr)?                                                                                       // |
    ;                                                                                                                 // |
                                                                                                                      // |
// Comparison operators                                                                                               // |
// Example --- == (equal to)                                                                                          // |
// Example --- != (not equal to)                                                                                      // |
// Example --- < (less than)                                                                                          // |
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

// Break statement - exits the nearest enclosing loop --------------------------------------------- [4]
// Example --- break;
breakStmt
    : BREAK
    ;

// Continue statement - skips to the next iteration of the nearest enclosing loop ----------------- [5]
// Example --- continue;
continueStmt
    : CONTINUE
    ;

// Return statement - exits a function with optional return value --------------------------------- [6]
// Example --- return;
// Example --- return result * 2;
returnStmt
    : RETURN expr?
    ;

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
// Example --- (x + y) (parenthesized expression)
primaryExpr
    : ID                         // Variable reference
    | INT                        // Integer literal
    | FLOAT                      // Floating-point literal
    | STRING_LITERAL             // String literal
    | BOOL_LITERAL               // Boolean literal
    | structInit                 // [8] Struct initialization
    | memberAccess               // [9] Struct field access
    | LEFTPAREN expr RIGHTPAREN  // Parenthesized expression
    ;

// Struct initialization - creates a new struct instance ------------------------------------------ [8]
// Example --- Person {name: "Marc", age: 24, is_dork: true}
structInit
    : ID LEFTBRACE structInitField (COMMA structInitField)* RIGHTBRACE
    ;


// Struct initialization field - sets an initial value for a struct field ------------------------- [8]
// Example --- name: "Marc"
// Example --- age: 24
structInitField
    : ID COLON expr
    ;

// Member access - retrieves a field from a struct ------------------------------------------------ [9]
// Example --- person.name
// Example --- company.address.zipCode
memberAccess
    : ID DOT ID (DOT ID)*  // Supports nested access (a.b.c)
    ;

// ============================== TYPE SYSTEM ================================

// Type - all possible data types in the language
// Example --- String
// Example --- u32
// Example --- Person (user-defined struct)
type
    : STRING      // String type
    | UINT        // Unsigned integer types (u8, u16, etc.)
    | INT_TYPE    // Signed integer types (i8, i16, etc.)
    | FLOAT_TYPE  // Floating-point types (f8, f16, etc.)
    | BOOL        // Boolean type
    | ID          // User-defined struct types
    ;

