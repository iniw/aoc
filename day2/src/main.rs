use std::{fs, str::Lines};

fn check_for_safety(levels: &[i32]) -> bool {
    let mut is_increasing = None::<bool>;
    let mut previous_level = None::<i32>;

    for level in levels.iter().copied() {
        if let Some(previous) = previous_level {
            match is_increasing.get_or_insert(level > previous) {
                true => {
                    if level <= previous || level - previous > 3 {
                        return false;
                    }
                }
                false => {
                    if level >= previous || previous - level > 3 {
                        return false;
                    }
                }
            }
        }
        previous_level = Some(level);
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

        if check_for_safety(&levels) {
            result += 1;
        } else {
            for i in 0..levels.len() {
                let filtered = {
                    let mut new = levels.clone();
                    new.remove(i);
                    new
                };

                if check_for_safety(&filtered) {
                    result += 1;
                    break;
                }
            }
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input!");
    dbg!(part1(input.lines()));
    dbg!(part2(input.lines()));
}
