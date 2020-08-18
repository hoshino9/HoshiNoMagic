#[macro_use]
extern crate pest;

use hnm::recog::{HNMParser, Rule};
use pest::{Parser};

macro_rules! simple_parse {
    ( $rule:ident , $input:expr ) => {
        {
            use hnm::recog::{HNMParser, Rule};

            parses_to! {
                parser: HNMParser,
                input: $input,
                rule: Rule::$rule,
                tokens: [
                    $rule(0, $input.len())
                ]
            }
        }
    };
}

fn debug(rule: Rule, input: &str) -> &str {
    println!("Debugging {:?}", input);
    let ps = HNMParser::parse(rule, input).unwrap();
    println!("{:#?}", ps);

    input
}

#[test]
fn magic_kw() {
    simple_parse!(MagicKW, "magic");
    simple_parse!(MagicKW, "mag");
}

#[test]
fn sym() {
    simple_parse!(MagicSym, "center");
    simple_parse!(MagicSym, "foo");
}

#[test]
#[should_panic]
fn fail_sym() {
    // simple_parse!(MagicSym, "foo+");
    // simple_parse!(MagicSym, "..?");
    //
    // simple_parse!(MagicSym, "center");
    // simple_parse!(MagicSym, "foo");
    parses_to! {
        parser: HNMParser,
        input: "i:",
        rule: Rule::MagicSym,
        tokens: [
            MagicSym(0, 2)
        ]
    }
    // simple_parse!(MagicSym, "i:");
}

#[test]
fn ty() {
    parses_to! {
        parser: HNMParser,
        input: debug(Rule::Type, "i32"),
        rule: Rule::Type,
        tokens: [
            Type(0, 3, [
                I32(0, 3)
            ])
        ]
    }

    parses_to! {
        parser: HNMParser,
        input: debug(Rule::Type, "logic"),
        rule: Rule::Type,
        tokens: [
            Type(0, 5, [
                Logic(0, 5)
            ])
        ]
    }

    parses_to! {
        parser: HNMParser,
        input: debug(Rule::Type, "lang"),
        rule: Rule::Type,
        tokens: [
            Type(0, 4, [
                Lang(0, 4)
            ])
        ]
    }

    parses_to! {
        parser: HNMParser,
        input: debug(Rule::Type, "letter"),
        rule: Rule::Type,
        tokens: [
            Type(0, 6, [
                Letter(0, 6)
            ])
        ]
    }

    parses_to! {
        parser: HNMParser,
        input: "~i32",
        rule: Rule::Type,
        tokens: [
            Type(0, 4, [
                Ref(0, 4, [
                    Type(1, 4, [
                        I32(1, 4)
                    ])
                ])
            ])
        ]
    }


    parses_to! {
        parser: HNMParser,
        input: "(~i32)****",
        rule: Rule::Type,
        tokens: [
            Type(0, 10, [
                Arr(0, 10, [
                    Ref(1, 5, [
                        Type(2, 5, [
                            I32(2, 5)
                        ])
                    ])
                ])
            ])
        ]
    }
}

#[test]
fn params() {
    parses_to! {
        parser: HNMParser,
        input: "i: i32 ; j: lang*",
        rule: Rule::params,
        tokens: [
            params(0, 17, [
                MagicSym(0, 1),
                Type(3, 6, [
                    I32(3, 6)
                ]),

                MagicSym(9, 10),
                Type(12, 17, [
                    Arr(12, 17, [
                        Lang(12, 16)
                    ])
                ])
            ])
        ]
    }
}

#[test]
fn box_decl() {
    debug(Rule::box_decl, "box i: i32 <- 1");

    parses_to! {
        parser: HNMParser,
        input: "box i: i32 <- 1",
        rule: Rule::box_decl,
        tokens: [
            box_decl(0, 15, [
                MagicSym(4, 5),
                Type(7, 10, [
                    I32(7, 10)
                ]),
                expr(14, 15, [
                    Literal(14, 15, [
                        Num(14, 15)
                    ])
                ])
            ])
        ]
    }

    debug(Rule::box_decl, "box j <- 1");

    parses_to! {
        parser: HNMParser,
        input: "box j <- 1",
        rule: Rule::box_decl,
        tokens: [
            box_decl(0, 10, [
                MagicSym(4, 5),
                expr(9, 10, [
                    Literal(9, 10, [
                        Num(9, 10)
                    ])
                ])
            ])
        ]
    }
}

#[test]
fn block() {
    parses_to! {
        parser: HNMParser,
        input: debug(Rule::block, "{ 123. }"),
        rule: Rule::block,
        tokens: [
            block(0, 8, [
                stmt(2, 6, [
                    expr(2, 5, [
                        Literal(2, 5, [
                            Num(2, 5)
                        ])
                    ])
                ])
            ])
        ]
    }

    parses_to! {
        parser: HNMParser,
        input: debug(Rule::block, "{ 123 }: i32"),
        rule: Rule::block,
        tokens: [
            block(0, 12, [
                expr(2, 5, [
                    Literal(2, 5, [
                        Num(2, 5)
                    ])
                ]),

                Type(9, 12, [
                    I32(9, 12)
                ])
            ])
        ]
    }
}

#[test]
fn magic() {
    debug(Rule::magic, "mag center [ argc: i32 ; argv: lang* ] { box i: i32 <- 0. i }: i32");

    parses_to! {
        parser: HNMParser,
        input: r#"mag center [ argc: i32 ; argv: lang* ] { box i: i32 <- 0. i }: i32"#,
        rule: Rule::magic,
        tokens: [
            magic(0, 66, [
                magic_decl(0, 38, [
                    MagicKW(0, 3),
                    MagicSym(4, 10),
                    params(13, 36, [
                        MagicSym(13, 17),
                        Type(19, 22, [
                            I32(19, 22)
                        ]),

                        MagicSym(25, 29),
                        Type(31, 36, [
                            Arr(31, 36, [
                                Lang(31, 35)
                            ])
                        ])
                    ])
                ]),

                block(39, 66, [
                    stmt(41, 57, [
                        box_decl(41, 56, [
                            MagicSym(45, 46),
                            Type(48, 51, [
                                I32(48, 51)
                            ]),

                            expr(55, 56, [
                                Literal(55, 56, [
                                    Num(55, 56)
                                ])
                            ])
                        ])
                    ]),

                    expr(58, 59, [
                        MagicSym(58, 59)
                    ]),

                    Type(63, 66, [
                        I32(63, 66)
                    ])
                ])
            ])
        ]
    }
}