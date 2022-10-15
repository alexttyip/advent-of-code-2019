use std::fs::File;
use std::io::{BufRead, BufReader};

fn fuel_for_fuel(mass: i32) -> i32 {
    if mass > 0 {
        mass + fuel_for_fuel(mass / 3 - 2)
    } else {
        0
    }
}

pub fn run() {
    let file = File::open("./inputs/day01.txt").unwrap();
    let reader = BufReader::new(file);
    let nums: Vec<i32> = reader.lines().flatten().flat_map(|l| l.parse()).collect();

    let part1 = nums.iter().fold(0, |acc, curr| acc + curr / 3 - 2);
    let part2 = nums.iter().fold(0, |acc, curr| acc + fuel_for_fuel(curr / 3 - 2));

    assert_eq!(part1, 3406432);
    assert_eq!(part2, 5106777);

    println!("--- Day 01 ---");
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    println!();
}
