#![feature(test)]

use std::time::Instant;

fn next_number(curr_number: &mut i32) {
    let mut changed = false;
    let mut digits: Vec<_> = (*curr_number + 1)
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

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

fn part1() -> i32 {
    let input: Vec<i32> = include_str!("./input.txt")
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

    part1
}

fn part2() -> i32 {
    let input: Vec<i32> = include_str!("./input.txt")
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

    part2
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 04 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 1099);
    assert_eq!(part2, 710);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
