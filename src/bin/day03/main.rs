#![feature(test)]

use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn cal_next_point(point: &mut (i32, i32), direction: &str) {
    match direction {
        "U" => point.1 += 1,
        "R" => point.0 += 1,
        "D" => point.1 -= 1,
        "L" => point.0 -= 1,
        _ => (),
    }
}

fn split_instruction(instruction: &String) -> (&str, i32) {
    let (direction, magnitude_string) = instruction.split_at(1);

    (direction, magnitude_string.parse().unwrap())
}

fn manhattan((x, y): &(i32, i32)) -> i32 {
    x.abs() + y.abs()
}

fn part1() -> i32 {
    let lines: Vec<Vec<String>> = include_str!("./input.txt")
        .lines()
        .map(|line| line.trim().split(",").map(str::to_string).collect())
        .collect();

    let mut points = HashMap::new();
    let mut point = (0, 0);
    let mut steps = 0;

    for instruction in &lines[0] {
        let (direction, magnitude) = split_instruction(instruction);

        for _ in 0..magnitude {
            cal_next_point(&mut point, direction);
            steps += 1;

            points.insert(point, steps);
        }
    }

    let mut part1 = HashSet::new();
    let mut part2 = Vec::new();
    point = (0, 0);
    steps = 0;

    for instruction in &lines[1] {
        let (direction, magnitude) = split_instruction(instruction);

        for _ in 0..magnitude {
            cal_next_point(&mut point, direction);
            steps += 1;

            if let Some(line_a_steps) = points.get(&point) {
                part1.insert(point);
                part2.push(line_a_steps + steps);
            }
        }
    }

    part1.iter().map(manhattan).min().unwrap()
}

fn part2() -> i32 {
    let lines: Vec<Vec<String>> = include_str!("./input.txt")
        .lines()
        .map(|line| line.trim().split(",").map(str::to_string).collect())
        .collect();

    let mut points = HashMap::new();
    let mut point = (0, 0);
    let mut steps = 0;

    for instruction in &lines[0] {
        let (direction, magnitude) = split_instruction(instruction);

        for _ in 0..magnitude {
            cal_next_point(&mut point, direction);
            steps += 1;

            points.insert(point, steps);
        }
    }

    let mut part1 = HashSet::new();
    let mut part2 = Vec::new();
    point = (0, 0);
    steps = 0;

    for instruction in &lines[1] {
        let (direction, magnitude) = split_instruction(instruction);

        for _ in 0..magnitude {
            cal_next_point(&mut point, direction);
            steps += 1;

            if let Some(line_a_steps) = points.get(&point) {
                part1.insert(point);
                part2.push(line_a_steps + steps);
            }
        }
    }

    *part2.iter().min().unwrap()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 03 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 260);
    assert_eq!(part2, 15612);
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
