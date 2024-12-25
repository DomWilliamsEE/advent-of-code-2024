use common::prelude::*;
use std::collections::HashMap;

pub struct Day24_2024;

impl Solution for Day24_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => parse(input).evaluate().to_string(),
            PartNumber::Part2 => solve_part2(input),
        }
    }
}

solution!(
    Day24_2024,
    [
        example_part1(
            4,
            "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
        ),
        example_part1(
            2024,
            "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"
        ),
        solution_part1(Some(59364044286798)),
        solution_part2(Some("cbj,cfk,dmn,gmt,qjj,z07,z18,z35")),
    ]
);

#[test]
fn test_build_2024_24() {}

// -----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct InitialState {
    values: HashMap<String, bool>,
    gates: HashMap<String, (String, Operation, String)>,
}

impl InitialState {
    fn to_graphviz(&self) -> String {
        let mut result = String::from("digraph G {\n");

        for (output, (input1, op, input2)) in &self.gates {
            result.push_str(&format!("    {input1} -> {output} [label=\"{op:?}\"]\n",));
            result.push_str(&format!("    {input2} -> {output} [label=\"{op:?}\"]\n",));
        }

        result.push('}');
        result
    }

    fn evaluate(mut self) -> i64 {
        fn eval(wire: String, state: &mut InitialState, depth: usize) -> bool {
            if let Some(val) = state.values.get(&wire) {
                return *val;
            }

            if depth > 5000 {
                println!("too deep");
                return false;
            }

            let (a, op, b) = state.gates[&wire].clone();
            let val = match op {
                Operation::And => eval(a, state, depth + 1) && eval(b, state, depth + 1),
                Operation::Or => eval(a, state, depth + 1) || eval(b, state, depth + 1),
                Operation::Xor => eval(a, state, depth + 1) ^ eval(b, state, depth + 1),
            };
            state.values.insert(wire, val);
            val
        }

        let mut res = 0;
        for i in 0.. {
            let z = format!("z{i:02}");
            if !self.gates.contains_key(&z) {
                break;
            }

            let val = eval(z.clone(), &mut self, 0);
            res |= (val as i64) << i;
        }

        res
    }

    fn read_register(&self, prefix: char) -> i64 {
        let mut res = 0;
        for i in 0.. {
            let node = format!("{prefix}{i:02}");
            if !self.values.contains_key(&node) {
                break;
            }

            let val = self.values[&node];
            res |= (val as i64) << i;
        }
        res
    }

    fn set_input(&mut self, prefix: char, value: i64) {
        for i in 0.. {
            let gate = format!("{prefix}{i:02}");
            if !self.gates.contains_key(&gate) {
                return;
            }

            let bit = (value >> i) & 1 == 1;
            self.values.insert(gate, bit);
        }
    }

    fn set_inputs(&mut self, x: i64, y: i64) {
        self.set_input('x', x);
        self.set_input('y', y);
    }

    fn is_correct(&self) -> bool {
        let state = self.clone();

        let a = state.read_register('x');
        let b = state.read_register('y');

        let res = state.evaluate();
        res == a + b
    }

    fn swap(&mut self, a: &str, b: &str) {
        let aval = self.gates.remove(a).unwrap();
        let bval = self.gates.remove(b).unwrap();

        self.gates.insert(a.to_string(), bval);
        self.gates.insert(b.to_string(), aval);
    }
}

fn parse(input: &str) -> InitialState {
    let mut lines = input.lines();
    let values = (&mut lines)
        .take_while(|l| !l.is_empty())
        .map(|l| l.split_once(": ").unwrap())
        .map(|(name, val)| {
            (
                name.trim().to_string(),
                match val.trim() {
                    "0" => false,
                    "1" => true,
                    _ => unreachable!("bad value {val}"),
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let gates = lines
        .map(|l| {
            let (lhs, rhs) = l.split_once(" -> ").unwrap();
            let mut words = lhs.split_whitespace();
            let a = words.next().unwrap();
            let operation = match words.next().unwrap() {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "XOR" => Operation::Xor,
                _ => unreachable!("bad operand {words:?}"),
            };
            let b = words.next().unwrap();

            (rhs.to_owned(), (a.to_string(), operation, b.to_string()))
        })
        .collect::<HashMap<_, _>>();

    InitialState { values, gates }
}

fn solve_part2(input: &str) -> String {
    let state = parse(input);
    assert!(!state.is_correct());

    let mut bad_gates = vec![];
    for (gate, (a, op, b)) in state.gates.iter() {
        if gate.starts_with('z') && gate != "z45" {
            if *op != Operation::Xor {
                bad_gates.push(gate.clone());
            }
        } else {
            if !a.starts_with('x') && !a.starts_with('y') {
                if *op == Operation::Xor {
                    bad_gates.push(gate.clone());
                }
            }
        }

        if matches!(op, Operation::Xor)
            && (a.starts_with('x') || b.starts_with('x'))
            && (a.starts_with('y') || b.starts_with('y'))
            && !gate.ends_with("00")
        {
            if !state
                .gates
                .iter()
                .any(|(_, (a, op, b))| matches!(op, Operation::Xor) && (a == gate || b == gate))
            {
                bad_gates.push(gate.clone());
            }
        }

        if matches!(op, Operation::And)
            && (a.starts_with('x') || b.starts_with('x'))
            && (a.starts_with('y') || b.starts_with('y'))
            && !gate.ends_with("00")
        {
            if !state
                .gates
                .iter()
                .any(|(_, (a, op, b))| matches!(op, Operation::Or) && (a == gate || b == gate))
            {
                bad_gates.push(gate.clone());
            }
        }
    }

    bad_gates.sort();
    bad_gates.dedup();
    println!("potential bad gates: {bad_gates:?}");
    // assert_eq!(bad_gates.len(), 8);

    let swaps = bad_gates
        .iter()
        .permutations(8)
        .map(|p| [(p[0], p[1]), (p[2], p[3]), (p[4], p[5]), (p[6], p[7])]);

    for swap in swaps {
        let mut state = state.clone();
        for pair in swap.iter() {
            state.swap(pair.0, pair.1);
            if state.is_correct() {
                return swap.iter().flat_map(|(a, b)| [a, b]).sorted().join(",");
            }
        }
    }

    unreachable!()
}
