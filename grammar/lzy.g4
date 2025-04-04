// =============================================================================
// lzy Language Grammar Definition
// =============================================================================

grammar Lzy;

options {tokenVocab=lzyLexer;}
import ParserStatements, ParserExpressions;

// =========================== TOP-LEVEL STRUCTURE =============================

// Root rule - a program consists of one or more functions or struct definitions
// Example --- struct Person { String name; u32 age; } void main() { ... }
program
    : (function | structDefinition)+ EOF # ProgramStructure
    ;

// Function definition - includes return type, name, parameters, and body
// Example --- i32 add(i32 a, i32 b) { return a + b; }
function
    : (VOID | rettype=type_t) id=ID LEFTPAREN params=paramList? RIGHTPAREN LEFTBRACE
      body_=statement*
      RIGHTBRACE
    ;

functionCall
    : ttype=type_t var=ID EQUAL func=ID LEFTPAREN (args+=expr (COMMA args+=expr)*)? RIGHTPAREN  # AssignFunctionCall
    | func=ID LEFTPAREN (args+=expr (COMMA args+=expr)*)? RIGHTPAREN                          # SimpleFunctionCall
    ;

// Function parameters - list of type-name pairs
// Example --- u32 age, String name, bool is_dork
paramList
    : paramTypes+=type_t paramNames+=ID (COMMA paramTypes+=type_t paramNames+=ID)*
    ;

// Struct definition - a named collection of fields
// Example --- struct Person { String name; u32 age; bool is_dork; }
structDefinition
    : STRUCT id=ID LEFTBRACE
      fields+=structField*
      RIGHTBRACE
    ;

// Fields within a struct - each has a type and name
// Example --- String name;
// Example --- u32 count;
structField
    : fieldType=type_t fieldName=ID SEMICOLON
    ;

// Struct initialization - creates a new struct instance ------------------------------------------ [8]
// Example --- Person {name: "Marc", age: 24, is_dork: true}
structInit
    : ID LEFTBRACE structInitField (COMMA fields+=structInitField)* RIGHTBRACE
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
    : object=ID DOT fields+=ID (DOT fields+=ID)*                   # VariableMemberAccess
    | array=arrayAccess DOT fields+=ID (DOT fields+=ID)*           # ArrayMemberAccess
    ;

// Array/Vector access - accesses an element in an array or vector by index
// Example --- a[0]
// Example --- points[i + 1]
// Example --- vector_name[3]
arrayAccess
    : array=ID (LEFTBRACKET indices+=expr RIGHTBRACKET)+  # ArrayIndexAccess
    ;

// Array initialization - creates a new array with values
// Example --- {1, 2, 3, 8}
arrayInit
    : LEFTBRACE ((expr | arrayInit)) (COMMA (expr | arrayInit))* RIGHTBRACE  # NonEmptyArrayInit
    | LEFTBRACE RIGHTBRACE  # EmptyArrayInit
    ;

// Vector initialization - creates a new vector with values (same syntax as array init)
// Example --- {3, 5, 2}
vectorInit
    : LEFTBRACE ((expr | vectorInit)) (COMMA (expr | vectorInit))* RIGHTBRACE  # NonEmptyVectorInit
    | LEFTBRACE RIGHTBRACE  # EmptyVectorInit
    ;

// ============================== TYPE SYSTEM ================================

// Type - all possible data types in the language
// Example --- String
// Example --- u32
// Example --- Person (user-defined struct)
// Example --- u8[]
// Example --- Vec<u8>
type_t
    : STRING                                    # StringType
    | UINT                                      # UintType
    | INT_TYPE                                  # IntType
    | FLOAT_TYPE                                # FloatType
    | BOOL                                      # BoolType
    | ID                                        # UserDefinedType
    | elemType=type_t LEFTBRACKET RIGHTBRACKET  # ArrayType
    | VEC LESS elemType=type_t GREATER          # VectorType
    ;
