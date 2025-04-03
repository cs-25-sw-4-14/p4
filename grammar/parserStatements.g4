grammar ParserStatements;

// ============================== STATEMENTS ==================================

// Statements - the basic units of execution in the language
statement
    : declaration SEMICOLON       # DeclarationStatement
    | assignment SEMICOLON        # AssignmentStatement
    | whileLoop                   # whileLoopStatement
    | ifStmt                      # IfStatement
    | breakStmt SEMICOLON         # breakStatement
    | continueStmt SEMICOLON      # ContinueStatement
    | returnStmt SEMICOLON        # ReturnStatement
    | expr SEMICOLON              # ExpressionStatement
    | functionCall SEMICOLON      # FunctionCallStatement
    | typeCasting SEMICOLON       # TypeCastingStatement
    ;

// Variable declaration - defines a new variable with optional initialization --------------------- [0]
// Example --- mut String name = "Marc";
// Example --- triv u32 age = 24;
// Example --- u8[] a;
// Example --- Vec<u8> a;
declaration
    : modifier* type_t ID (EQUAL (expr | arrayInit | vectorInit))?
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
    : (ID | memberAccess | arrayAccess) EQUAL expr
    ;

typeCasting
    : targetType=type_t targetVar=ID EQUAL LEFTPAREN sourceType=type_t RIGHTPAREN sourceVar=ID
    ;

// While loop - executes statements as long as condition is true ---------------------------------- [2]
// Example --- while (i < 10) { i = i + 1; sum = sum + i; }
whileLoop
    : WHILE LEFTPAREN condition=expr RIGHTPAREN LEFTBRACE
      body=statement*
      RIGHTBRACE
    ;

// If statement - conditional execution with optional else-if and else blocks --------------------- [3]
// Example --- if (x > 10) { return "high"; } else if (x > 5) { return "medium"; } else { return "low"; }
ifStmt
    : IF LEFTPAREN condition=expr RIGHTPAREN LEFTBRACE
      thenBranch=statement*
      RIGHTBRACE
      elseIfBranches=elseIfStmt*
      elseBranch=elseStmt?
    ;

// Else-if statement - additional conditions checked if previous conditions are false ------------- [3]
// Example --- else if (x > 5) { return "medium"; }
elseIfStmt
    : ELSE IF LEFTPAREN condition=expr RIGHTPAREN LEFTBRACE
      body=statement*
      RIGHTBRACE
    ;

// Else statement - executed if all previous conditions are false --------------------------------- [3]
// Example --- else { return "low"; }
elseStmt
    : ELSE LEFTBRACE
      body=statement*
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
    : RETURN value=expr?
    ;
