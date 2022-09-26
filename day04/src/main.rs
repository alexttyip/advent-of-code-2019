use std::fs;

fn next_number(curr_number: &mut i32) {
    let mut changed = false;
    let mut digits: Vec<_> = (*curr_number + 1).to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

    for i in 1..digits.len() {
        if changed || digits[i] < digits[i - 1] {
            changed = true;

            digits[i] = digits[i - 1];
        }
    }

    *curr_number = digits.iter().fold(0, |acc, curr| acc * 10 + (*curr as i32));
}

fn process_number(curr_number_string: &mut String, part1: &mut i32, part2: &mut i32) {
    let mut chars = curr_number_string.chars();

    let mut prev_char = chars.next().unwrap();
    let mut repeated = false;
    let mut repeated_twice = false;
    let mut repeat = 0;

    while let Some(curr_char) = chars.next() {
        if curr_char == prev_char {
            repeat += 1;
            repeated = true;
        } else if repeat == 1 {
            repeated_twice = true;
            break;
        } else {
            repeat = 0;
        }

        prev_char = curr_char;
    }

    if repeated_twice || repeat == 1 {
        *part1 += 1;
        *part2 += 1;
    } else if repeated {
        *part1 += 1;
    }
}

fn main() {
    let input: Vec<i32> = fs::read_to_string("./input.txt")
        .unwrap()
        .trim()
        .split("-")
        .map(|bound| bound.parse().unwrap())
        .collect();

    let mut current_number = input[0];
    let mut part1 = 0;
    let mut part2 = 0;

    while current_number < input[1] {
        process_number(&mut current_number.to_string(), &mut part1, &mut part2);
        next_number(&mut current_number);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
