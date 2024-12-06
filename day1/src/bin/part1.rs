use std::fs;

use day1::parse_columns;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");

    let (col1, col2) = parse_columns(input.lines());

    let answer = col1
        .into_iter()
        .zip(col2)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    println!("Part 1 = {}", answer);
}
