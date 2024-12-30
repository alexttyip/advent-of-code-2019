#![feature(test)]

use std::fs;
use std::time::Instant;

fn run_computer(noun: usize, verb: usize) -> usize {
    let mut input: Vec<usize> = include_str!("input.txt")
        .split(",")
        .flat_map(|s| s.parse())
        .collect();

    input[1] = noun;
    input[2] = verb;

    let mut pointer = 0;

    loop {
        let opcode = input[pointer];
        let a_idx = input[pointer + 1];
        let b_idx = input[pointer + 2];
        let c_idx = input[pointer + 3];

        let a = input[a_idx];
        let b = input[b_idx];

        match opcode {
            1 => {
                input[c_idx] = a + b;
                pointer += 4;
            }
            2 => {
                input[c_idx] = a * b;
                pointer += 4;
            }
            _ => {
                break;
            }
        }
    }

    input[0]
}

fn part1() -> usize {
    run_computer(12, 2)
}

fn part2() -> usize {
    for i in 0..100 {
        for j in 0..100 {
            if run_computer(i, j) == 19690720 {
                return 100 * i + j;
            }
        }
    }

    panic!()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 02 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 5482655);
    assert_eq!(part2, 4967);
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
