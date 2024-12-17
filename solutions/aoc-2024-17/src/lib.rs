use common::prelude::*;
use hashbrown::HashMap;
use std::ops::BitXor;

pub struct Day17_2024;

impl Solution for Day17_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => parse(input).compute_fully().into_iter().join(","),
            PartNumber::Part2 => find_correct_a_register(input).to_string(),
        }
    }
}

const PART1_EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

const PART1_EXAMPLE_SIMPLE: &str = "Register A: 10
Register B: 0
Register C: 0
Program: 5,0,5,1,5,4";

const PART2_EXAMPLE: &str = "Register A: 2024
Register B: 0
Register C: 0
Program: 0,3,5,4,3,0";

solution!(
    Day17_2024,
    [
        example_part1("4,6,3,5,6,3,5,2,1,0", PART1_EXAMPLE),
        example_part1("0,1,2", PART1_EXAMPLE_SIMPLE),
        solution_part1(Some("3,5,0,1,5,1,5,1,0")),
        example_part2(117440, PART2_EXAMPLE),
        solution_part2(Some(107413700225434)),
    ]
);

#[test]
fn test_build_2024_17() {}

// -----

struct Program {
    registers: [i64; 3],
    insns: Vec<u8>,
}

fn parse(program: &str) -> Program {
    let mut registers = [0; 3];
    let lines = lines(program).collect_vec();
    assert_eq!(lines.len(), 4);

    for i in 0..3 {
        registers[i] = lines[i].split_once(':').unwrap().1.trim().parse().unwrap();
    }

    let insns = lines[3]
        .split_once(':')
        .unwrap()
        .1
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect_vec();
    Program { registers, insns }
}

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
enum Register {
    A = 0,
    B,
    C,
}

impl Program {
    fn reg(&self, reg: Register) -> i64 {
        self.registers[reg as usize]
    }

    fn set_reg(&mut self, reg: Register, value: i64) {
        self.registers[reg as usize] = value;
        // println!("set reg {reg:?} to {value}");
    }

    fn combo(&self, val: i64) -> i64 {
        match val {
            0..=3 => val,
            4 => self.reg(Register::A),
            5 => self.reg(Register::B),
            6 => self.reg(Register::C),
            _ => unreachable!(),
        }
    }

    fn adv(&mut self, output_reg: Register, operand: i64) {
        let num = self.reg(Register::A);
        let denom = 2_i64.pow(u32::try_from(self.combo(operand)).expect("too big A for divide"));
        let res = num / denom;
        self.set_reg(output_reg, res);
    }

    fn compute_fully(&mut self) -> Vec<u8> {
        let mut output = vec![];

        let mut pc = 0;
        loop {
            let Some(insn) = self.insns.get(pc as usize).copied() else {
                break;
            };

            let operand = || self.insns[pc as usize + 1] as i64;

            // println!("state: {:?}", self.registers);
            // println!("insn: {insn}, operand: {:?}", self.insns.get(pc + 1));

            let mut jumped = false;
            match insn {
                0 => {
                    self.adv(Register::A, operand());
                }

                1 => {
                    let op = operand();
                    let res = op.bitxor(self.reg(Register::B));
                    self.set_reg(Register::B, res);
                }

                2 => {
                    let val = self.combo(operand()) % 8;
                    self.set_reg(Register::B, val);
                }

                3 => {
                    if self.reg(Register::A) != 0 {
                        pc = operand() as u8;
                        jumped = true;
                    }
                }
                4 => {
                    let a = self.reg(Register::B);
                    let b = self.reg(Register::C);
                    self.set_reg(Register::B, a.bitxor(b));
                }
                5 => {
                    let val = self.combo(operand()) % 8;
                    output.push(val as u64 as u8);
                    // println!("output: {output:?}")
                }
                6 => {
                    self.adv(Register::B, operand());
                }
                7 => {
                    self.adv(Register::C, operand());
                }
                _ => unreachable!("invalid insn"),
            }

            if !jumped {
                pc += 2;
            }
        }

        output
    }

    fn hash(&mut self, i: i64) -> Vec<u8> {
        let program = self;
        program.registers = [i, 0, 0];
        program.compute_fully()
    }
}
fn find_correct_a_register(program: &str) -> i64 {
    let mut program = parse(program);
    let expected = program.insns.clone();

    let mut candidates = HashMap::new();

    for digits_to_check in 1..=expected.len() {
        let mut new_candidates = vec![];
        for base in candidates
            .get_mut(&(digits_to_check - 1))
            .cloned()
            .unwrap_or_else(|| (0..8).collect_vec())
        {
            for i in 0..8 {
                let input = 8 * base + i;
                let val = program.hash(input);
                if val.len() == digits_to_check {
                    if expected
                        .iter()
                        .skip(expected.len() - digits_to_check)
                        .zip(val.iter())
                        .all(|(a, b)| a == b)
                    {
                        println!(
                            "nice for base {base} and i {i}, input {input} == last digits {val:?}"
                        );
                        new_candidates.push(input);
                    }
                }
            }
        }
        candidates.insert(digits_to_check, new_candidates);
    }

    println!("{candidates:?}");
    let best = *candidates
        .get(&expected.len())
        .unwrap()
        .iter()
        .min()
        .unwrap();

    let res = program.hash(best);
    println!("best {best}, res {res:?}, expected {expected:?}");
    assert_eq!(res, expected);
    best
}
