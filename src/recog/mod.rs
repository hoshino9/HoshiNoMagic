// use nom::{
//     bytes::complete::{tag, take_while},
//     branch::alt,
//     IResult
// };
//
// pub type Source<'lt> = &'lt str;
// pub type HResult<'s, O> = IResult<&'s str, O>;
//
// const KEY_CHAR: &'static str = "()[]{}";
//
// fn hard_kw() -> Vec<&'static str> {
//     vec!["magic", "mag", "box"]
// }
//
// fn mag_sym() -> Vec<char> {
//     vec!['_', '?']
// }
//
// fn is_mag_char(c: char) -> bool {
//     c.is_alphabetic() || mag_sym().contains(&c)
// }
//
// fn mag_kw(source: &str) -> HResult<&str> {
//     alt((tag("magic"), tag("mag")))(source)
// }
//
// fn mag_str(source: &str) -> HResult<&str> {
//     take_while(is_mag_char)(source)
// }
//
// fn magic(source: &str) -> HResult<&str> {
//     source.split_ascii_whitespace();
//     let (source, _) = mag_kw(source)?;
//     let (source, name) = mag_str(source)?;
//
//     todo!()
// }

#[derive(Parser)]
#[grammar = "hm.pest"]
pub struct HNMParser;