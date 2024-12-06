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
            (_, FirstNumber) => parse_number(&input, &mut lexer, begin).map(|n| {
                numbers.0 = n;
                Comma
            }),
            (',', Comma) => Some(SecondNumber),
            (_, SecondNumber) => parse_number(&input, &mut lexer, begin).map(|n| {
                numbers.1 = n;
                CloseParen
            }),
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
