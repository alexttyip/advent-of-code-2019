use std::fs;

fn run(input: isize, part2: bool) {
    let mut code: Vec<isize> = fs::read_to_string("./input.txt").unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut pointer = 0;

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
                    println!("{}", code[a_idx]);
                }

                if part2 { break; }

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
                code[c_idx] = if code[a_idx] < code[b_idx] {
                    1
                } else {
                    0
                };

                pointer += 4;
            }
            8 => {
                code[c_idx] = if code[a_idx] == code[b_idx] {
                    1
                } else {
                    0
                };

                pointer += 4;
            }
            99 => { break; }
            _ => ()
        }
    }
}

fn main() {
    println!("Part 1:");
    run(1, false);
    println!();
    println!("Part 2:");
    run(5, true);
    println!();
}