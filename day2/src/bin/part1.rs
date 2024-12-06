use std::fs;

use day2::check_for_safety;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");

    let mut answer = 0;

    for line in input.lines() {
        let levels = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().expect("Failed to parse number from input"))
            .collect::<Vec<_>>();

        if check_for_safety(&levels) {
            answer += 1;
        }
    }

    println!("Part 1 = {}", answer);
}
