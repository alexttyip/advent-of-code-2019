use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn cal_next_point(point: &mut (i32, i32), direction: &str) {
    match direction {
        "U" => point.1 += 1,
        "R" => point.0 += 1,
        "D" => point.1 -= 1,
        "L" => point.0 -= 1,
        _ => ()
    }
}

fn split_instruction(instruction: &String) -> (&str, i32) {
    let (direction, magnitude_string) = instruction.split_at(1);

    (direction, magnitude_string.parse().unwrap())
}

fn manhattan((x, y): &(i32, i32)) -> i32 {
    x.abs() + y.abs()
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let lines: Vec<Vec<String>> = BufReader::new(file).lines()
        .map(|line| line.unwrap()
            .trim()
            .split(",")
            .map(str::to_string)
            .collect()
        )
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

    println!("Part 1: {}", part1.iter().map(manhattan).min().unwrap());
    println!("Part 2: {}", part2.iter().min().unwrap());
}
