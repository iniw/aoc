use day3::parse_number;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");

    let mut muls: Vec<(u32, u32)> = Vec::new();
    let mut lexer = input.char_indices().peekable();

    let mut state = State::Enabled(Enabled::Idle);
    let mut numbers = (0, 0);

    while let Some((begin, c)) = lexer.next() {
        state = match state {
            State::Enabled(s) => match (c, s) {
                ('m', Enabled::Idle) => Some(Enabled::Mul(Mul::U)),
                ('d', Enabled::Idle) => Some(Enabled::Dont(Dont::O)),

                (c, Enabled::Mul(s)) => match (c, s) {
                    ('u', Mul::U) => Some(Mul::L),
                    ('l', Mul::L) => Some(Mul::OpenParen),
                    ('(', Mul::OpenParen) => Some(Mul::FirstNumber),
                    (_, Mul::FirstNumber) => parse_number(&input, &mut lexer, begin).map(|n| {
                        numbers.0 = n;
                        Mul::Comma
                    }),
                    (',', Mul::Comma) => Some(Mul::SecondNumber),
                    (_, Mul::SecondNumber) => parse_number(&input, &mut lexer, begin).map(|n| {
                        numbers.1 = n;
                        Mul::CloseParen
                    }),
                    (')', Mul::CloseParen) => {
                        muls.push(numbers);
                        state = State::Enabled(Enabled::Idle);
                        continue;
                    }

                    _ => None,
                }
                .map(Enabled::Mul),

                (c, Enabled::Dont(s)) => match (c, s) {
                    ('o', Dont::O) => Some(Dont::N),
                    ('n', Dont::N) => Some(Dont::Quote),
                    ('\'', Dont::Quote) => Some(Dont::T),
                    ('t', Dont::T) => Some(Dont::OpenParen),
                    ('(', Dont::OpenParen) => Some(Dont::CloseParen),
                    (')', Dont::CloseParen) => {
                        state = State::Disabled(Disabled::Idle);
                        continue;
                    }

                    _ => None,
                }
                .map(Enabled::Dont),

                _ => Some(Enabled::Idle),
            }
            .map(State::Enabled)
            .unwrap_or(State::Enabled(Enabled::Idle)),

            State::Disabled(s) => match (c, s) {
                ('d', Disabled::Idle) => Some(Disabled::Do(Do::O)),

                (c, Disabled::Do(s)) => match (c, s) {
                    ('o', Do::O) => Some(Do::OpenParen),
                    ('(', Do::OpenParen) => Some(Do::CloseParen),
                    (')', Do::CloseParen) => {
                        state = State::Enabled(Enabled::Idle);
                        continue;
                    }

                    _ => None,
                }
                .map(Disabled::Do),

                _ => Some(Disabled::Idle),
            }
            .map(State::Disabled)
            .unwrap_or(State::Disabled(Disabled::Idle)),
        };
    }

    let answer = muls.into_iter().map(|(a, b)| a * b).sum::<u32>();
    println!("Part 2 = {}", answer);
}

enum State {
    Enabled(Enabled),
    Disabled(Disabled),
}

enum Enabled {
    Idle,
    Mul(Mul),
    Dont(Dont),
}

enum Disabled {
    Idle,
    Do(Do),
}

enum Mul {
    U,
    L,
    OpenParen,
    FirstNumber,
    Comma,
    SecondNumber,
    CloseParen,
}

enum Dont {
    O,
    N,
    Quote,
    T,
    OpenParen,
    CloseParen,
}

enum Do {
    O,
    OpenParen,
    CloseParen,
}
