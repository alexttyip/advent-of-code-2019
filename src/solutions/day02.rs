use std::fs;

fn run_computer(noun: usize, verb: usize) -> usize {
    let mut input: Vec<usize> = fs::read_to_string("./inputs/day02.txt")
        .unwrap()
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
            _ => { break; }
        }
    }

    input[0]
}

fn cal_part2() -> usize {
    for i in 0..100 {
        for j in 0..100 {
            if run_computer(i, j) == 19690720 {
                return 100 * i + j;
            }
        }
    }

    0
}


pub fn run() {
    let part1 = run_computer(12, 2);
    let part2 = cal_part2();

    assert_eq!(part1, 5482655);
    assert_eq!(part2, 4967);

    println!("--- Day 02 ---");
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    println!();
}