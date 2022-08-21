use std::fs::{File};
use std::io::{BufRead, BufReader};

fn fuel_for_fuel(mass: i32) -> i32 {
    if mass > 0 {
        mass + fuel_for_fuel(mass / 3 - 2)
    } else {
        0
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut part1 = 0;
    let mut part2 = 0;

    for x in reader.lines() {
        let i: i32 = x.unwrap().parse().unwrap();
        let fuel = i / 3 - 2;
        part1 += fuel;
        part2 += fuel_for_fuel(fuel);
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
