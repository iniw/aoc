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
        } else {
            for i in 0..levels.len() {
                let filtered = {
                    let mut new = levels.clone();
                    new.remove(i);
                    new
                };

                if check_for_safety(&filtered) {
                    answer += 1;
                    break;
                }
            }
        }
    }

    println!("Part 2 = {}", answer);
}
