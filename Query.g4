grammar Query;

query   : SELECT columns
          FROM source=identifier
          whereClause?
          limitClause?
           EOF;

columns : column (',' column)*;

column  : name=identifier (AS alias=identifier)?
        ;

whereClause
        : WHERE name=identifier
        ;

limitClause
        : LIMIT limit=NUMERIC_LITERAL
        ;

identifier
        :  IDENTIFIER
        ;

SELECT  : S E L E C T;

WHERE   : W H E R E;
AS      : A S;
FROM    : F R O M;
LIMIT   : L I M I T;

NUMERIC_LITERAL
        : DIGIT+
        ;

IDENTIFIER
        : [a-zA-Z_] [a-zA-Z_0-9]*
        ;

SPACES  : [ \t\r\n] -> skip
        ;

fragment DIGIT      : [0-9];


fragment A  : ('a'|'A');
fragment B  : ('b'|'B');
fragment C  : ('c'|'C');
fragment D  : ('d'|'D');
fragment E  : ('e'|'E');
fragment F  : ('f'|'F');
fragment G  : ('g'|'G');
fragment H  : ('h'|'H');
fragment I  : ('i'|'I');
fragment J  : ('j'|'J');
fragment K  : ('k'|'K');
fragment L  : ('l'|'L');
fragment M  : ('m'|'M');
fragment N  : ('n'|'N');
fragment O  : ('o'|'O');
fragment P  : ('p'|'P');
fragment Q  : ('q'|'Q');
fragment R  : ('r'|'R');
fragment S  : ('s'|'S');
fragment T  : ('t'|'T');
fragment U  : ('u'|'U');
fragment V  : ('v'|'V');
fragment W  : ('w'|'W');
fragment X  : ('x'|'X');
fragment Y  : ('y'|'Y');
fragment Z  : ('z'|'Z');