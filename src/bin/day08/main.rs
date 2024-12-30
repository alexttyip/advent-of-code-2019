#![feature(test)]

use std::time::Instant;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

fn read_input() -> Vec<u32> {
    include_str!("./input.txt")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn part1() -> usize {
    let input = read_input();
    let layers: Vec<&[u32]> = input.chunks(IMAGE_WIDTH * IMAGE_HEIGHT).collect();

    let min_zero_index = layers
        .iter()
        .map(|layer| layer.iter().filter(|&p| *p == 0).count())
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();

    layers[min_zero_index].iter().filter(|&p| *p == 1).count()
        * layers[min_zero_index].iter().filter(|&p| *p == 2).count()
}

fn part2() -> [u32; IMAGE_WIDTH * IMAGE_HEIGHT] {
    let mut image = [2; IMAGE_WIDTH * IMAGE_HEIGHT];

    read_input()
        .iter()
        .enumerate()
        .map(|(i, v)| (i % (IMAGE_WIDTH * IMAGE_HEIGHT), v))
        .for_each(|(i, v)| {
            if image[i] == 2 {
                image[i] = *v
            }
        });

    image
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 08 ---");
    println!("Part 1: {}", part1);

    for row in part2.chunks(IMAGE_WIDTH) {
        for pixel in row {
            if pixel == &0 {
                print!(" ");
            } else {
                print!("#");
            }
        }

        println!();
    }
    println!();

    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 2413);
    println!("Correct answer for part 2 is: BCPZB")
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
