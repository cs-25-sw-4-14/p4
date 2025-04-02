grammar ParserStatements;
options {tokenVocab=LzyLexer;}

// ============================== STATEMENTS ==================================

// Statements - the basic units of execution in the language
statement
    : declaration SEMICOLON       # DeclarationStmt
    | assignment SEMICOLON        # AssignmentStmt
    | whileLoop                   # WhileLoopStmt
    | ifStatement                 # IfStmt
    | breakStmt SEMICOLON         # BreakStmt
    | continueStmt SEMICOLON      # ContinueStmt
    | returnStmt SEMICOLON        # ReturnStmt
    | expr SEMICOLON              # ExpressionStmt
    | functionCall SEMICOLON      # FunctionCallStmt
    | typeCasting SEMICOLON       # TypeCastingStmt
    ;

// Variable declaration - defines a new variable with optional initialization --------------------- [0]
// Example --- mut String name = "Marc";
// Example --- triv u32 age = 24;
// Example --- u8[] a;
// Example --- Vec<u8> a;
declaration
    : mods+=modifier* varType=type_t varName=ID (EQUAL initialValue=(expr | arrayInit | vectorInit))? # VariableDeclaration
    ;

// Variable modifiers - affects variable behavior (e.g., mutability) ------------------------------ [0]
// Example --- mut (for mutable variables)
// Example --- triv (for constants/immutable variables)
modifier
    : TRIV    # TrivModifier
    | MUT     # MutModifier
    ;

// Assignment - sets a value to an existing variable, struct field, or array/vector element ------------- [1] 
// Example --- count = 10;
// Example --- person.age = 30;
// Example --- a[0] = 42;
// Example --- vec[1] = 5;
assignment
    : target=(ID | memberAccess | arrayAccess) EQUAL value=expr # VariableAssignment
    ;

typeCasting
    : targetType=type_t targetVar=ID EQUAL LEFTPAREN sourceType=type_t RIGHTPAREN sourceVar=ID # TypeCast;

// While loop - executes statements as long as condition is true ---------------------------------- [2]
// Example --- while (i < 10) { i = i + 1; sum = sum + i; }
whileLoop
    : WHILE LEFTPAREN condition=expr RIGHTPAREN LEFTBRACE
      body=statement*
      RIGHTBRACE # WhileLoop
    ;

// If statement - conditional execution with optional else-if and else blocks --------------------- [3]
// Example --- if (x > 10) { return "high"; } else if (x > 5) { return "medium"; } else { return "low"; }
ifStatement
    : IF LEFTPAREN condition=expr RIGHTPAREN LEFTBRACE
      thenBranch=statement*
      RIGHTBRACE
      elseIfBranches=elseIfStatement*
      elseBranch=elseStatement? # IfStatement
    ;

// Else-if statement - additional conditions checked if previous conditions are false ------------- [3]
// Example --- else if (x > 5) { return "medium"; }
elseIfStatement
    : ELSE IF LEFTPAREN condition=expr RIGHTPAREN LEFTBRACE
      body=statement*
      RIGHTBRACE # ElseIfStatement
    ;

// Else statement - executed if all previous conditions are false --------------------------------- [3]
// Example --- else { return "low"; }
elseStatement
    : ELSE LEFTBRACE
      body=statement*
      RIGHTBRACE # ElseStatement
    ;

// Break statement - exits the nearest enclosing loop --------------------------------------------- [4]
// Example --- break;
breakStmt
    : BREAK # BreakStatement
    ;

// Continue statement - skips to the next iteration of the nearest enclosing loop ----------------- [5]
// Example --- continue;
continueStmt
    : CONTINUE # ContinueStatement
    ;

// Return statement - exits a function with optional return value --------------------------------- [6]
// Example --- return;
// Example --- return result * 2;
returnStmt
    : RETURN value=expr? # ReturnStatement
    ;
