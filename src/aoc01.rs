use std::fs;

pub fn run() {
    println!("AOC 2023 - 01");
    let contents =
        fs::read_to_string("inputs/01_input.txt").expect("Something went wrong reading the file");

    let mut sum = 0;
    for line in contents.lines() {
        sum += sum_with_text_numbers(line);
    }

    println!("Sum: {}", sum);
}

fn sum_numbers_on_line(line: &str) -> i32 {
    // find the first number
    let mut start = '0';
    for (_, c) in line.chars().enumerate() {
        if c.is_numeric() {
            start = c;
            break;
        }
    }

    // find the last number
    let mut end = '0';
    for (_, c) in line.chars().enumerate() {
        if c.is_numeric() {
            end = c;
        }
    }

    // concatenate the numbers
    let mut number = String::new();
    number.push(start);
    number.push(end);
    let number: i32 = number.parse().unwrap();
    number
}

fn sum_with_text_numbers(line: &str) -> i32 {
    let numbers = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut indices = Vec::new();
    for (word, value) in &numbers {
        // check multiple occurences
        let mut index = line.find(word);
        while let Some(i) = index {
            indices.push((i, *value));
            if (i + 1) >= line.len() {
                break;
            }
            index = line[i + 1..].find(word).map(|j| i + j + 1);
        }
    }

    // find indices of the numbers
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            let value = c.to_digit(10).unwrap();
            indices.push((i, value));
        }
    }

    // sort the indices
    indices.sort_by(|a, b| a.0.cmp(&b.0));

    // return first and last value
    let mut result = String::new();
    let first = indices.first().unwrap();
    let last = indices.last().unwrap();
    result.push_str(&first.1.to_string());
    result.push_str(&last.1.to_string());
    print!("{} ", result);

    // concatenate the numbers
    let number: i32 = result.parse().unwrap();
    number
}
