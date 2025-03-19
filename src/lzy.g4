grammar lzy;

program:
 //   (def)+ EOF |
 //   (stmt)+ EOF;
;

expr:
    eqExpr ('||' eqExpr)*;

eqExpr:
    relBigExpr (('==' | '!=') relBigExpr)*;

relBigExpr:
    relSmallExpr ('>' relSmallExpr)*;

relSmallExpr:
    plusExpr ('<' plusExpr)*;



arithExpr:
    minusExpr;

minusExpr:
    plusExpr ('-' plusExpr)*;

plusExpr:
    divExpr ('+' divExpr)*;

divExpr:
    multExpr ('/' multExpr)*;

multExpr:
    modExpr ('*' modExpr)*;

modExpr:
    type ('%' type)*;





boolTerm:
    //'(' boolExpr ')' |
    'true' |
    'false'|
    ID;

aritExpr:
    aritTerm |
    aritTerm '+' aritTerm |
    aritTerm '-' aritTerm |
    aritTerm '*' aritTerm |
    aritTerm '/' aritTerm |
    aritTerm '%' aritTerm |
    aritTerm '^' aritTerm |
    aritTerm '<<' aritTerm |
    aritTerm '>>' aritTerm |
    aritTerm '&' aritTerm |
    aritTerm '|' aritTerm |
    aritTerm '~' aritTerm;

aritTerm:
    '(' aritExpr ')' |
    type |
    ID;

type:
    'u8'    |
    'u16'   |
    'u32'   |
    'u64'   |
    'i8'    |
    'i16'   |
    'i32'   |
    'i64'   |
    'f8'    |
    'f16'   |
    'f32'   |
    'f64'   |
    'bool'  |
    'string';

mutability:
    'mut';

importance:
    'triv';



ID: [a-zA-Z][a-zA-Z0-9]*;
STRING: '"' .*? '"';
FLOAT: [0-9]* '.' [0-9]+;
INT: '0' | [1-9] [0-9]*;
WS: [ \t\r\n]+ -> skip;

