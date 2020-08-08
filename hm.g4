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

// keywords
MAGIC_KW: 'magic' | 'mag';
TYPE: Bool | I32 | Str | Letter;
HUMAN_BOOL: Ja | Nein;

// literal values
HUMAN_NUM: '-'? DIGIT+;
HUMAN_LETTER: '\'' HUMAN_RAW_LETTER_ESC '\'';
HUMAN_LANG: '"' HUMAN_RAW_LETTER_ESC* '"';

// symbol
SYMBOL: MAG_SYM_CHAR (DIGIT | MAG_SYM_CHAR)* '?'?;


// parsers
literal: HUMAN_BOOL | HUMAN_NUM | HUMAN_LETTER | HUMAN_LANG;
right_value: literal | SYMBOL;

box_expr: 'box' ('<' TYPE '>')? SYMBOL '<-' right_value;

stmt: box_expr;

magic: MAGIC_KW sym=SYMBOL '[' ']';

// ignores
WS: (' ' | '\\' [ntr]) -> skip;