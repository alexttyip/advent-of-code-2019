#![feature(test)]

use std::time::Instant;

fn run_computer(input: isize, part2: bool) -> isize {
    let mut code: Vec<isize> = include_str!("input.txt")
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pointer = 0;

    let mut output = 0;

    loop {
        let mut first_value = code[pointer];

        let opcode = first_value % 100;
        first_value /= 100;
        let a_mode = first_value % 10;
        first_value /= 10;
        let b_mode = first_value % 10;
        first_value /= 10;
        let c_mode = first_value % 10;

        let a_idx: usize = if a_mode == 0 {
            usize::try_from(code[pointer + 1]).ok().unwrap()
        } else {
            pointer + 1
        };

        let b_idx: usize = if b_mode == 0 {
            usize::try_from(code[pointer + 2]).ok().unwrap()
        } else {
            pointer + 2
        };

        let c_idx: usize = if c_mode == 0 {
            usize::try_from(code[pointer + 3]).ok().unwrap()
        } else {
            pointer + 3
        };

        match opcode {
            1 => {
                code[c_idx] = code[a_idx] + code[b_idx];
                pointer += 4;
            }
            2 => {
                code[c_idx] = code[a_idx] * code[b_idx];
                pointer += 4;
            }
            3 => {
                code[a_idx] = input;
                pointer += 2;
            }
            4 => {
                if code[a_idx] != 0 {
                    output = output * 10 + code[a_idx];
                }

                if part2 {
                    break;
                }

                pointer += 2;
            }
            5 => {
                pointer = if code[a_idx] != 0 {
                    usize::try_from(code[b_idx]).ok().unwrap()
                } else {
                    pointer + 3
                };
            }
            6 => {
                pointer = if code[a_idx] == 0 {
                    usize::try_from(code[b_idx]).ok().unwrap()
                } else {
                    pointer + 3
                };
            }
            7 => {
                code[c_idx] = if code[a_idx] < code[b_idx] { 1 } else { 0 };

                pointer += 4;
            }
            8 => {
                code[c_idx] = if code[a_idx] == code[b_idx] { 1 } else { 0 };

                pointer += 4;
            }
            99 => {
                break;
            }
            _ => (),
        }
    }

    output
}

fn part1() -> isize {
    run_computer(1, false)
}

fn part2() -> isize {
    run_computer(5, true)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 05 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 13210611);
    assert_eq!(part2, 584126);
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
