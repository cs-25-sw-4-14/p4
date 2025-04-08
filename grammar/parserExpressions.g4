grammar ParserExpressions;

// ============================== EXPRESSIONS =================================

// Expression - the building block for conditions and values -------------------------------------- [7]
// Example --- a + b * c
expr
    : expression=logicalOrExpr
    ;

// Logical OR expression - combines AND expressions with OR --------------------------------------- [7]
// Example --- isAdmin || hasSpecialPermission
logicalOrExpr
    : left=logicalAndExpr                             # LogicalOrSingle
    | left=logicalOrExpr op=OR right=logicalAndExpr   # LogicalOrExpression
    ;

// Logical AND expression - combines equality expressions with AND -------------------------------- [7]
// Example --- isLoggedIn && hasPermission
logicalAndExpr
    : left=equalityExpr                              # LogicalAndSingle
    | left=logicalAndExpr op=AND right=equalityExpr  # LogicalAndExpression
    ;

// Equality expression - compares expressions for equality/inequality ----------------------------- [7]
// Example --- username == "admin"
// Example --- status != "error"
equalityExpr
    : left=relationalExpr                                        # EqualitySingle
    | left=equalityExpr op=(EQUAL_EQUAL | NOT_EQUAL) right=relationalExpr  # EqualityExpression
    ;

// Relational expression - compares expressions with ordering operators --------------------------- [7]
// Example --- age > 18
// Example --- score <= 100
relationalExpr
    : left=additiveExpr                                                      # RelationalSingle
    | left=relationalExpr op=(LESS | LESS_EQUAL | GREATER | GREATER_EQUAL) right=additiveExpr  # RelationalExpression
    ;

// Additive expression - performs addition and subtraction ---------------------------------------- [7]
// Example --- x + y
// Example --- total - discount
additiveExpr
    : left=multiplicativeExpr                               # AdditiveSingle
    | left=additiveExpr op=(PLUS | MINUS) right=multiplicativeExpr  # AdditiveExpression
    ;

// Multiplicative expression - performs multiplication, division, and modulo ---------------------- [7]
// Example --- width * height
// Example --- total / count
// Example --- value % 10
multiplicativeExpr
    : left=unaryExpr                                   # MultiplicativeSingle
    | left=multiplicativeExpr op=(MUL | DIV | MOD) right=unaryExpr  # MultiplicativeExpression
    ;

// Unary expression - applies unary operators to expressions -------------------------------------- [7]
// Example --- -x
// Example --- !isValid
unaryExpr
    : op=(MINUS | NOT) operand=unaryExpr  # UnaryOp
    | primaryExpr                         # UnaryPrimary
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
    : ID                                # VariableRef
    | INT                               # IntLiteral
    | FLOAT                             # FloatLiteral
    | STRING_LITERAL                    # StringLiteral
    | CHAR_LITERAL                      # CharLiteral
    | BOOL_LITERAL                      # BoolLiteral
    | structInit                        # StructInitExpr
    | arrayInit                         # ArrayInitExpr
    | vectorInit                        # VectorInitExpr
    | memberAccess                      # MemberAccessExpr
    | arrayAccess                       # ArrayAccessExpr
    | functionCall                      # FunctionCallExpr
    | LEFTPAREN inner=expr RIGHTPAREN   # ParenExpr
    ;
