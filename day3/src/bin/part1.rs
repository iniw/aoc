use std::fs;

use day3::parse_number;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    let mut muls: Vec<(u32, u32)> = Vec::new();
    let mut lexer = input.char_indices().peekable();

    let mut state = M;
    let mut numbers = (0, 0);

    while let Some((begin, c)) = lexer.next() {
        let new_state = match (c, state) {
            ('m', M) => Some(U),
            ('u', U) => Some(L),
            ('l', L) => Some(OpenParen),
            ('(', OpenParen) => Some(FirstNumber),
            ('0'..='9', FirstNumber) => {
                numbers.0 = parse_number(&mut lexer, &input, begin);
                Some(Comma)
            }
            (',', Comma) => Some(SecondNumber),
            ('0'..='9', SecondNumber) => {
                numbers.1 = parse_number(&mut lexer, &input, begin);
                Some(CloseParen)
            }
            (')', CloseParen) => {
                muls.push(numbers);
                None
            }
            _ => None,
        };
        state = new_state.unwrap_or(M);
    }

    let answer = muls.into_iter().map(|(a, b)| a * b).sum::<u32>();
    println!("Part 1 = {}", answer);
}

enum State {
    M,
    U,
    L,
    OpenParen,
    FirstNumber,
    Comma,
    SecondNumber,
    CloseParen,
}

use State::*;
