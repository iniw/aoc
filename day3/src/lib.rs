use std::{iter::Peekable, str::CharIndices};

pub fn parse_number<'a>(
    lexer: &mut Peekable<CharIndices<'a>>,
    input: &'a str,
    begin: usize,
) -> u32 {
    while lexer.next_if(|(_, c)| c.is_ascii_digit()).is_some() {}

    let str = if let Some((end, _)) = lexer.peek() {
        &input[begin..*end]
    } else {
        &input[begin..]
    };

    str.parse().expect("Failed to parse number from input")
}
