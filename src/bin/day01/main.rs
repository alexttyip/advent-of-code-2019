#![feature(test)]

use std::time::Instant;

fn fuel_for_fuel(mass: i32) -> i32 {
    if mass > 0 {
        mass + fuel_for_fuel(mass / 3 - 2)
    } else {
        0
    }
}

fn part1() -> i32 {
    let nums: Vec<i32> = include_str!("input.txt")
        .lines()
        .flat_map(|l| l.parse())
        .collect();

    nums.iter().fold(0, |acc, curr| acc + curr / 3 - 2)
}

fn part2() -> i32 {
    let nums: Vec<i32> = include_str!("input.txt")
        .lines()
        .flat_map(|l| l.parse())
        .collect();

    nums.iter()
        .fold(0, |acc, curr| acc + fuel_for_fuel(curr / 3 - 2))
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 01 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 3406432);
    assert_eq!(part2, 5106777);
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
