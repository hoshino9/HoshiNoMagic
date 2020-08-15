#[macro_use]
extern crate pest;

use hnm::regon::{HNMParser, Rule};

macro_rules! simple_parse {
    ( $rule:ident , $input:expr ) => {
        {
            use hnm::regon::{HNMParser, Rule};

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
    simple_parse!(Type, "i32");
    simple_parse!(Type, "logic");
    simple_parse!(Type, "lang");
    simple_parse!(Type, "letter");

    parses_to! {
        parser: HNMParser,
        input: "~i32",
        rule: Rule::Type,
        tokens: [
            Type(0, 4, [
                Type(1, 4)
            ])
        ]
    }


    parses_to! {
        parser: HNMParser,
        input: "~i32****",
        rule: Rule::Type,
        tokens: [
            Type(0, 4, [
                Type(1, 4)
            ])
        ]
    }
}

#[test]
fn params() {
    parses_to! {
        parser: HNMParser,
        input: "i: i32",
        rule: Rule::params,
        tokens: [
            params(0, 6, [
                MagicSym(0, 1),
                Type(3, 6)
            ])
        ]
    }
}

#[test]
fn box_decl() {
    parses_to! {
        parser: HNMParser,
        input: "box i: i32 <- 1",
        rule: Rule::box_decl,
        tokens: [
            box_decl(0, 15, [
                MagicSym(4, 5),
                Type(7, 10),
                expr(14, 15, [
                    Literal(14, 15)
                ])
            ])
        ]
    }

    parses_to! {
        parser: HNMParser,
        input: "box j <- 1",
        rule: Rule::box_decl,
        tokens: [
            box_decl(0, 10, [
                MagicSym(4, 5),
                expr(9, 10, [
                    Literal(9, 10)
                ])
            ])
        ]
    }
}