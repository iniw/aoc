use std::{fs, str::Lines};

fn check_for_safety(levels: &[i32]) -> bool {
    let mut is_increasing = None::<bool>;
    let mut previous_number = None::<i32>;

    for number in levels.iter().copied() {
        if let Some(n) = previous_number {
            match is_increasing.get_or_insert(number > n) {
                true => {
                    if number <= n || number - n > 3 {
                        return false;
                    }
                }
                false => {
                    if number >= n || n - number > 3 {
                        return false;
                    }
                }
            }
        }
        previous_number = Some(number);
    }

    true
}

fn part1(input: Lines) -> i32 {
    let mut result = 0;

    for line in input {
        let levels = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().expect("Failed to parse number from input"))
            .collect::<Vec<_>>();

        if check_for_safety(&levels) {
            result += 1;
        }
    }

    result
}

fn part2(input: Lines) -> i32 {
    let mut result = 0;

    for line in input {
        let levels = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().expect("Failed to parse number from input"))
            .collect::<Vec<_>>();

        if !check_for_safety(&levels) {
            for i in 0..levels.len() {
                let mut filtered = levels.clone();
                filtered.remove(i);

                if check_for_safety(&filtered) {
                    result += 1;
                    break;
                }
            }
        } else {
            result += 1;
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    dbg!(part1(input.lines()));
    dbg!(part2(input.lines()));
}
