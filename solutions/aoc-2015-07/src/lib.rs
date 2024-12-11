use collect_array::CollectArrayResult;
use common::itertools::Itertools;
use common::{lines, solution, PartNumber, Solution, SolutionInput};
use std::collections::HashMap;

pub struct Day07_2015;

impl Solution for Day07_2015 {
    fn solve(input: &str, part: PartNumber) -> i64 {
        match part {
            PartNumber::Part1 => {
                let mut state = State::default();
                state.run_instructions(input);
                state.values["a"] as i64
            }
            PartNumber::Part2 => {
                let mut state = State::default();
                state.values.insert("b".into(), 46065);
                state.run_instructions(input);
                state.values["a"] as i64
            }
        }
    }
}
solution!(
    Day07_2015,
    [
        (PartNumber::Part1, SolutionInput::FullInput, Some(46065)),
        (PartNumber::Part2, SolutionInput::FullInput, Some(14134)),
    ]
);

#[test]
fn test_build_2015_07() {}

// -----

#[derive(Debug, Clone)]
enum WireSource {
    Var(String),
    Not(String),
    And(String, String),
    Or(String, String),
    Lshift(String, String),
    Rshift(String, String),
}

#[derive(Default)]
struct State {
    // wire name -> its source
    circuit: HashMap<String, WireSource>,
    values: HashMap<String, u16>,
}

impl State {
    fn run_instructions(&mut self, input: &str) {
        for line in lines(input) {
            let [op, dst] = line
                .split(" -> ")
                .map(|s| s.trim())
                .collect::<CollectArrayResult<_, 2>>()
                .unwrap();

            let split = op.split(" ").map(|s| s.trim()).collect_vec();
            let op = match split.len() {
                1 => WireSource::Var(split[0].to_owned()),
                2 if split[0] == "NOT" => WireSource::Not(split[1].to_owned()),
                3 if split[1] == "AND" => WireSource::And(split[0].to_owned(), split[2].to_owned()),
                3 if split[1] == "OR" => WireSource::Or(split[0].to_owned(), split[2].to_owned()),
                3 if split[1] == "LSHIFT" => {
                    WireSource::Lshift(split[0].to_owned(), split[2].parse().unwrap())
                }
                3 if split[1] == "RSHIFT" => {
                    WireSource::Rshift(split[0].to_owned(), split[2].parse().unwrap())
                }
                _ => panic!("bad instruction: {}", line),
            };

            assert!(self.circuit.insert(dst.to_owned(), op).is_none());
        }

        println!("circuit: {:#?}", self.circuit);

        let all_keys = self.circuit.keys().map(|s| s.to_owned()).collect_vec();
        for wire in all_keys.iter().map(|s| s.as_str()) {
            // println!("evaluating {}, values: {:#?}", wire, state.values);
            if self.values.contains_key(wire) {
                continue;
            }

            let val = self.eval(wire);
            println!("evaluated {} to {}", wire, val);
            self.values.insert(wire.to_owned(), val);
        }

        println!("final values: {:#?}", self.values);
    }

    fn eval(&mut self, wire: &str) -> u16 {
        if let Ok(val) = wire.parse::<u16>() {
            return val;
        }

        if let Some(val) = self.values.get(wire).copied() {
            return val;
        }

        let source = self
            .circuit
            .get(wire)
            .cloned()
            .unwrap_or_else(|| panic!("no source found for {wire}"));

        let val = match source {
            WireSource::Var(name) => {
                let a = self.eval(&name);
                a
            }

            WireSource::And(a, b) => {
                let a = self.eval(&a);
                let b = self.eval(&b);
                a & b
            }

            WireSource::Or(a, b) => {
                let a = self.eval(&a);
                let b = self.eval(&b);
                a | b
            }

            WireSource::Lshift(a, b) => {
                let a = self.eval(&a);
                let b = self.eval(&b);
                a.wrapping_shl(b as u32)
            }

            WireSource::Rshift(a, b) => {
                let a = self.eval(&a);
                let b = self.eval(&b);
                a.wrapping_shr(b as u32)
            }

            WireSource::Not(a) => {
                let a = self.eval(&a);
                !a
            }
        };

        self.values.insert(wire.to_owned(), val);
        val
    }
}

#[test]
fn test_example() {
    let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    let mut state = State::default();
    state.run_instructions(input);
    let vals = state.values;

    // d: 72
    // e: 507
    // f: 492
    // g: 114
    // h: 65412
    // i: 65079
    // x: 123
    // y: 456

    println!("{:?}", vals);
    assert_eq!(vals["d"], 72_u16);
    assert_eq!(vals["e"], 507_u16);
    assert_eq!(vals["f"], 492_u16);
    assert_eq!(vals["g"], 114_u16);
    assert_eq!(vals["h"], 65412_u16);
    assert_eq!(vals["i"], 65079_u16);
    assert_eq!(vals["x"], 123_u16);
    assert_eq!(vals["y"], 456_u16);
}
