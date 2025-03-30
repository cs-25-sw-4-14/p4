grammar parserStatements;
options {tokenVocab=lzyLexer;}

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
    | functionCall SEMICOLON      // TODO FUNCTIONCALL COMMENT
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
    : (ID | memberAccess) EQUAL expr
    ;

// While loop - executes statements as long as condition is true ---------------------------------- [2]
// Example --- while (i < 10) { i = i + 1; sum = sum + i; }
whileLoop
    : WHILE LEFTPAREN expr RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
    ;

// If statement - conditional execution with optional else-if and else blocks --------------------- [3]
// Example --- if (x > 10) { return "high"; } else if (x > 5) { return "medium"; } else { return "low"; }
ifStatement
    : IF LEFTPAREN expr RIGHTPAREN LEFTBRACE
      statement*
      RIGHTBRACE
      elseIfStatement*
      elseStatement?
    ;

// Else-if statement - additional conditions checked if previous conditions are false ------------- [3]
// Example --- else if (x > 5) { return "medium"; }
elseIfStatement
    : ELSE IF LEFTPAREN expr RIGHTPAREN LEFTBRACE
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