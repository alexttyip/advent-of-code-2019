use std::fs;
use std::cmp::max;
use std::collections::VecDeque;

struct Amplifier {
    code: Vec<isize>,
    pointer: usize,
    input: VecDeque<isize>,
    output: isize,
}

fn read_input() -> Vec<isize> {
    fs::read_to_string("./input.txt").unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_idx(mode: isize, pointer: usize, code: Vec<isize>, offset: usize) -> usize {
    if mode == 0 {
        usize::try_from(code[pointer + offset]).ok().unwrap()
    } else {
        pointer + offset
    }
}

fn get_a_idx(a_mode: isize, pointer: usize, code: Vec<isize>) -> usize {
    get_idx(a_mode, pointer, code, 1)
}

fn get_b_idx(b_mode: isize, pointer: usize, code: Vec<isize>) -> usize {
    get_idx(b_mode, pointer, code, 2)
}

fn get_c_idx(c_mode: isize, pointer: usize, code: Vec<isize>) -> usize {
    get_idx(c_mode, pointer, code, 3)
}

fn run(amplifier: Amplifier) -> Option<Amplifier> {
    let mut code = amplifier.code;
    let mut pointer = amplifier.pointer;
    let mut inputs = amplifier.input;

    loop {
        let mut first_value = code[pointer];

        let opcode = first_value % 100;
        first_value /= 100;
        let a_mode = first_value % 10;
        first_value /= 10;
        let b_mode = first_value % 10;
        first_value /= 10;
        let c_mode = first_value % 10;

        match opcode {
            1 => {
                let a_idx = get_a_idx(a_mode, pointer, code.clone());
                let b_idx = get_b_idx(b_mode, pointer, code.clone());
                let c_idx = get_c_idx(c_mode, pointer, code.clone());
                code[c_idx] = code[a_idx] + code[b_idx];
                pointer += 4;
            }
            2 => {
                let a_idx = get_a_idx(a_mode, pointer, code.clone());
                let b_idx = get_b_idx(b_mode, pointer, code.clone());
                let c_idx = get_c_idx(c_mode, pointer, code.clone());
                code[c_idx] = code[a_idx] * code[b_idx];
                pointer += 4;
            }
            3 => {
                let a_idx = get_a_idx(a_mode, pointer, code.clone());
                code[a_idx] = inputs.pop_front().unwrap();

                pointer += 2;
            }
            4 => {
                let a_idx = get_a_idx(a_mode, pointer, code.clone());

                return Some(Amplifier {
                    code: code.clone(),
                    pointer: pointer + 2,
                    input: VecDeque::new(),
                    output: code[a_idx],
                });
            }
            5 => {
                let a_idx = get_a_idx(a_mode, pointer, code.clone());
                let b_idx = get_b_idx(b_mode, pointer, code.clone());
                pointer = if code[a_idx] != 0 {
                    usize::try_from(code[b_idx]).ok().unwrap()
                } else {
                    pointer + 3
                };
            }
            6 => {
                let a_idx = get_a_idx(a_mode, pointer, code.clone());
                let b_idx = get_b_idx(b_mode, pointer, code.clone());
                pointer = if code[a_idx] == 0 {
                    usize::try_from(code[b_idx]).ok().unwrap()
                } else {
                    pointer + 3
                };
            }
            7 => {
                let a_idx = get_a_idx(a_mode, pointer, code.clone());
                let b_idx = get_b_idx(b_mode, pointer, code.clone());
                let c_idx = get_c_idx(c_mode, pointer, code.clone());

                code[c_idx] = (code[a_idx] < code[b_idx]) as isize;

                pointer += 4;
            }
            8 => {
                let a_idx = get_a_idx(a_mode, pointer, code.clone());
                let b_idx = get_b_idx(b_mode, pointer, code.clone());
                let c_idx = get_c_idx(c_mode, pointer, code.clone());

                code[c_idx] = (code[a_idx] == code[b_idx]) as isize;

                pointer += 4;
            }
            _ => {
                return None;
            }
        }
    }
}

fn permutations(from: usize, to: usize) -> Permutations {
    Permutations { idxs: (from..to).collect(), swaps: vec![0; to - from], i: 0 }
}

struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() { return None; }
                if self.swaps[self.i] < self.i { break; }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}

fn part1() -> isize {
    let code = read_input();

    let mut answer = 0;
    for perm in permutations(0, 5) {
        let mut output = 0;
        for i in perm {
            output = run(Amplifier {
                input: VecDeque::from([i as isize, output]),
                code: code.clone(),
                pointer: 0,
                output: 0,
            }).unwrap().output;
        }

        answer = max(answer, output);
    }

    answer
}

fn part2() -> isize {
    let code = read_input();

    let mut answer = 0;

    for perm in permutations(5, 10) {
        let mut amps: VecDeque<Amplifier> = VecDeque::from_iter(
            perm.iter().map(|i| Amplifier {
                code: code.clone(),
                pointer: 0,
                input: VecDeque::from([*i as isize]),
                output: 0,
            })
        );

        let mut output = 0;
        loop {
            let mut amp = amps.pop_front().unwrap();
            amp.input.push_back(output);

            match run(amp) {
                Some(new_amp) => {
                    output = new_amp.output;
                    amps.push_back(new_amp);
                }
                None => {
                    break;
                }
            }
        }

        answer = max(answer, output);
    }

    answer
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}