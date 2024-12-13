use common::prelude::*;
use regex::Regex;

pub struct Day13_2024;

impl Solution for Day13_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => ClawMachine::parse_lines(input)
                .into_iter()
                .filter_map(|machine| {
                    let slow = machine.find_cost_to_win_smol_brain();
                    let fast = machine.find_cost_to_win_big_brain(Some(100));

                    assert_eq!(slow, fast);

                    fast
                })
                .sum::<i64>(),
            PartNumber::Part2 => ClawMachine::parse_lines(input)
                .into_iter()
                .filter_map(|machine| {
                    machine
                        .with_added(10000000000000)
                        .find_cost_to_win_big_brain(None)
                })
                .sum::<i64>(),
        }
    }
}

solution!(
    Day13_2024,
    [
        solution_part1(Some(36758_i64)),
        example_part1(480, PART1_EXAMPLE),
        solution_part2(Some(76358113886726)),
        example_part2(875318608908, PART1_EXAMPLE),
    ]
);

const PART1_EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

#[test]
fn test_build_2024_13() {}

// -----

#[derive(Debug)]
struct ClawMachine {
    a: [i64; 2],
    b: [i64; 2],
    prize: [i64; 2],
}

impl ClawMachine {
    fn parse_lines(input: &str) -> Vec<Self> {
        let button_regex = Regex::new(r"X\+([0-9]+), Y\+([0-9]+)").unwrap();
        let prize_regex = Regex::new(r"X=([0-9]+), Y=([0-9]+)").unwrap();

        let two_ints = |s, regex: &Regex| -> [i64; 2] {
            let captures = regex.captures(s).unwrap();

            [
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
            ]
        };

        lines(input)
            .chunks(3)
            .into_iter()
            .map(|mut lines| Self {
                a: two_ints(lines.next().unwrap(), &button_regex),
                b: two_ints(lines.next().unwrap(), &button_regex),
                prize: two_ints(lines.next().unwrap(), &prize_regex),
            })
            .collect()
    }

    fn with_added(mut self, add: i64) -> Self {
        self.prize[0] += add;
        self.prize[1] += add;

        self
    }

    fn find_cost_to_win_smol_brain(&self) -> Option<i64> {
        let a_add = self.a;
        let b_add = self.b;
        let result = self.prize;

        match (0..=100).cartesian_product(0..=100).find_map(|(a, b)| {
            let res = [a * a_add[0] + b * b_add[0], a * a_add[1] + b * b_add[1]];
            if res == result {
                println!("smol brain says {a} a and {b} b");
                Some([a, b])
            } else {
                None
            }
        }) {
            Some([a, b]) => {
                println!("smol brain says {a} a and {b} b");
                Some(a * 3 + b)
            }
            None => {
                println!("smol brain says impossibru");
                None
            }
        }
    }

    fn find_cost_to_win_big_brain(&self, max_presses: Option<i64>) -> Option<i64> {
        // X = Ca + Db
        // Y = Ea + Fb

        let [x, y] = self.prize;
        let [c, e] = self.a;
        let [d, f] = self.b;

        let b = ((e * x) - (c * y)) / ((e * d) - (c * f));
        let a = (x - (d * b)) / c;

        let calc_x = (c * a) + (d * b);
        let calc_y = (e * a) + (f * b);

        if calc_x != self.prize[0]
            || calc_y != self.prize[1]
            || a > max_presses.unwrap_or(i64::MAX)
            || b > max_presses.unwrap_or(i64::MAX)
        {
            println!("big brain says impossibru");
            return None;
        }

        println!("big brain says {a} a and {b} b");
        Some(a * 3 + b)
    }
}

#[test]
fn test_parse() {
    let machines = ClawMachine::parse_lines(PART1_EXAMPLE);
    for machine in machines {
        println!(
            "{machine:?} => {:?}",
            machine.find_cost_to_win_big_brain(Some(100))
        );
    }
}
