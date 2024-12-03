use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Input doesn't exist");
    let lines = input.lines();

    let mut col1 = vec![];
    let mut col2 = vec![];
    for line in lines {
        for (i, number) in line.split_whitespace().enumerate() {
            let number = number
                .parse::<i32>()
                .expect("Failed to parse number in input");

            if i % 2 == 0 {
                col1.push(number);
            } else {
                col2.push(number);
            }
        }
    }

    col1.sort();
    col2.sort();

    let part1 = col1
        .iter()
        .zip(&col2)
        .map(|(a, b)| (a - *b).abs())
        .sum::<i32>();

    dbg!(part1);

    let part2 = col1
        .iter()
        .map(|a| a * col2.iter().filter(|b| a == *b).count() as i32)
        .sum::<i32>();

    dbg!(part2);
}
