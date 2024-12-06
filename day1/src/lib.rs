use std::str::Lines;

pub fn parse_columns(lines: Lines) -> (Vec<i32>, Vec<i32>) {
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

    (col1, col2)
}
