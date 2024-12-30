#![feature(test)]

use std::collections::{HashMap, VecDeque};
use std::time::Instant;

fn part1() -> u32 {
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();

    for line in include_str!("./input.txt").lines() {
        let orbited_and_orbiter: Vec<String> = line.split(")").map(String::from).collect();
        let orbited = orbited_and_orbiter[0].to_string();
        let orbiter = orbited_and_orbiter[1].to_string();

        orbits.entry(orbited).or_insert(vec![]).push(orbiter);
    }

    let mut objects: HashMap<&str, u32> = HashMap::from([("COM", 0)]);

    let mut queue: VecDeque<&str> = VecDeque::from(["COM"]);
    let mut count = 0;

    while let Some(curr_name) = queue.pop_front() {
        if let Some(new_orbits) = orbits.get(&curr_name.to_string()) {
            let orbit_count = objects[curr_name] + 1;

            for orbit in new_orbits.iter() {
                objects.insert(orbit, orbit_count);
                count += orbit_count;
                queue.push_back(orbit);
            }
        }
    }

    count
}

fn part2() -> usize {
    let mut orbits: HashMap<String, String> = HashMap::new();

    for line in include_str!("./input.txt").lines() {
        let pair: Vec<String> = line.split(")").map(String::from).collect();
        let parent = pair[0].to_string();
        let child = pair[1].to_string();

        orbits.insert(child, parent);
    }

    let mut my_ancestors: Vec<String> = Vec::new();
    let mut curr = "YOU";

    while let Some(new) = orbits.get(curr) {
        my_ancestors.push(new.to_string());
        curr = new;
    }

    curr = "SAN";
    let mut count = 0;

    while let Some(new) = orbits.get(curr) {
        match my_ancestors.iter().position(|s| s == new) {
            Some(idx) => {
                count += idx;
                break;
            }
            None => {
                count += 1;
            }
        }

        curr = new;
    }

    count
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 06 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 308790);
    assert_eq!(part2, 472);
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
