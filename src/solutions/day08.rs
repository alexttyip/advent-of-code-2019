use std::fs;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

fn read_input() -> Vec<u32> {
    fs::read_to_string("./inputs/day08.txt").unwrap()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn part1() -> usize {
    let input = read_input();
    let layers: Vec<&[u32]> = input.chunks(IMAGE_WIDTH * IMAGE_HEIGHT).collect();

    let min_zero_index = layers.iter()
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

    read_input().iter().enumerate()
        .map(|(i, v)| (i % (IMAGE_WIDTH * IMAGE_HEIGHT), v))
        .for_each(|(i, v)|
            if image[i] == 2 { image[i] = *v }
        );

    image
}

pub fn run() {
    println!("--- Day 08 ---");
    println!("Part 1: {}", part1());
    println!("Part 2:");

    let part2 = part2();

    for row in part2.chunks(IMAGE_WIDTH) {
        for pixel in row {
            if pixel == &0 {
                print!("⬛️");
            } else {
                print!("⬜️️");
            }
        }

        println!();
    }
    println!();
}