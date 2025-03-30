// =============================================================================
// lzy Language Grammar Definition
// =============================================================================

grammar lzy;

options {tokenVocab=lzyLexer;}
import parserStatements, parserExpressions;

// =========================== TOP-LEVEL STRUCTURE =============================

// Root rule - a program consists of one or more functions or struct definitions
// Example --- struct Person { String name; u32 age; } void main() { ... }
program
    : (function | structDefinition)+ EOF
    ;

// Function definition - includes return type, name, parameters, and body
// Example --- i32 add(i32 a, i32 b) { return a + b; }
function
    : (VOID | type) ID LEFTPAREN paramList? RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
    ;


functionCall
    : type ID EQUAL ID LEFTPAREN (ID (COMMA ID)*)? RIGHTPAREN
    | ID LEFTPAREN (ID (COMMA ID)*)? RIGHTPAREN
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

