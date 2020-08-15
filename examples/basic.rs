use hnm::recog::{HNMParser, Rule};
use pest::{Parser};
use pest::iterators::Tokens;

fn main() {
    let result = HNMParser::parse(Rule::magic, r#"mag qwq [ i: i32 ; j: lang ] {}: i32"#).unwrap();
    let tks: Tokens<Rule> = result.tokens();

    for tk in tks {
        println!("{:?}", tk);
    }
}