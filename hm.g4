grammar hm;

fragment HUMAN_RAW_LETTER: [a-zA-Z_0-9 ];
fragment HUMAN_RAW_LETTER_ESC: HUMAN_RAW_LETTER | ESCAPE;
fragment ESCAPE: '?' [?ntr"'];
fragment MAG_SYM_CHAR: [a-zA-Z_];
fragment DIGIT: [0-9];
fragment Ja: 'ja' | 'Ja';
fragment Nein: 'nein' | 'Nein';
fragment Bool: 'logic';
fragment I32: 'i32';
fragment Str: 'lang';
fragment Letter: 'letter';
fragment Ref: '~' TYPE;
fragment ArrayType: BaseType '*'*;

// keywords
MAGIC_KW: 'magic' | 'mag';
AREA: 'area';
HUMAN_BOOL: Ja | Nein;

fragment BaseType: Bool | I32 | Str | Letter | Ref ;
TYPE: BaseType | ArrayType;

// literal values
HUMAN_NUM: '-'? DIGIT+;
HUMAN_LETTER: '\'' HUMAN_RAW_LETTER_ESC '\'';
HUMAN_LANG: '"' HUMAN_RAW_LETTER_ESC* '"';

// symbol
SYMBOL: MAG_SYM_CHAR (DIGIT | MAG_SYM_CHAR)* '?'?;

// parsers
// area
path: head=SYMBOL ('->' tail=SYMBOL)*;
area_use: AREA area=path '.';

// literals
literal: HUMAN_BOOL | HUMAN_NUM | HUMAN_LETTER | HUMAN_LANG;
ref_expr: '~' expr;
deref_expr: '`' expr;

type_decl: ':' type=TYPE;

// exprs
expr: literal | SYMBOL | ref_expr | deref_expr | magic_call;
box_expr: 'box' SYMBOL (type=type_decl)? '<-' init=expr;

// stmts
stmte: (box_expr | expr);
stmt: stmte '.';

// magic decl or invoke
params: SYMBOL type_decl (';' SYMBOL type_decl)* ';'?;
varargs: expr (',' expr)*;
args: varargs? (';' varargs?)*;

// magic
magic: MAGIC_KW sym=SYMBOL '[' params? ']' ('{' body=stmt* '}' | '{' body=stmt* ret=expr '}' ':' ret_type=TYPE);
magic_call: mag=SYMBOL '[' args ']';

// item
item: area_use | magic;

// test
test: item* EOF;

// ignores
WS: [ \n\t\r] -> skip;