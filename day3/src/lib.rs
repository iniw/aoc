use std::{iter::Peekable, str::CharIndices};

pub fn parse_number<'a>(
    input: &'a str,
    lexer: &mut Peekable<CharIndices<'a>>,
    begin: usize,
) -> Option<u32> {
    while lexer.next_if(|(_, c)| c.is_ascii_digit()).is_some() {}
    let str = if let Some((end, _)) = lexer.peek() {
        &input[begin..*end]
    } else {
        &input[begin..]
    };

    if str.is_empty() {
        None
    } else {
        Some(str.parse().expect("Failed to parse number from input"))
    }
}
