use itertools::{repeat_n, Itertools};
use std::borrow::Cow;
use std::fs;

const PART1_EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
// leave blank if same as part1
const PART2_EXAMPLE: &str = "";

#[derive(Debug)]
struct Equation {
    result: i64,
    values: Vec<i64>,
}

fn eval_expr(values: &[i64], ops: &[&str]) -> i64 {
    let mut result = values[0];
    for (val, op) in values.iter().skip(1).zip(ops.iter()) {
        match *op {
            "+" => result += *val,
            "*" => result *= *val,
            "||" => {
                result = (result.to_string() + &val.to_string())
                    .parse::<i64>()
                    .unwrap();
            }
            _ => unreachable!(),
        }
    }

    result
}

#[test]
fn test_eval() {
    assert_eq!(eval_expr(&[10, 19], &["*"]), 190);
    assert_eq!(eval_expr(&[10, 19], &["+"]), 29);
    assert_eq!(eval_expr(&[81, 40, 27], &["*", "+"]), 3267);

    assert_eq!(eval_expr(&[6, 8, 6, 15], &["*", "||", "*"]), 7290);
}

impl Equation {
    fn parse(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let mut nums = line
                    .split(&[' ', ':'])
                    .filter(|s| !s.is_empty())
                    .map(|s| s.trim().parse::<i64>().unwrap());
                let result = nums.next().unwrap();
                Equation {
                    result,
                    values: nums.collect(),
                }
            })
            .collect()
    }

    fn is_solvable(&self, ops: &[&str]) -> bool {
        println!("{:?}", self);
        let combos = repeat_n(ops.iter().copied(), self.values.len() - 1).multi_cartesian_product();

        // println!(
        //     "combos for {:?}: {:?}",
        //     equation,
        //     combos.clone().collect_vec()
        // );

        for combo in combos {
            // println!("combo {:?}", combo);
            let last_value = self.values.last().unwrap();
            let vals = self.values.iter().zip(combo.iter());

            let mut expr = vals.map(|(i, op)| format!("{i} {op}")).join(" ");
            expr.push_str(&format!(" {}", last_value));
            // println!("expr {}", expr);

            let result = eval_expr(&self.values, &combo);
            // println!("expr {} == {result}", expr);
            if result == self.result {
                // println!("correct");
                return true;
            }
        }

        false
    }
}

fn part_one(input: &str) -> i64 {
    let equations = Equation::parse(input);

    let ops = ["*", "+"];
    let mut correct_sum = 0;
    for equation in equations {
        if equation.is_solvable(&ops) {
            println!("correct {equation:?}");
            correct_sum += equation.result;
        }
    }
    correct_sum
}

fn part_two(input: &str) -> i64 {
    let equations = Equation::parse(input);

    let ops = ["*", "+", "||"];
    let mut correct_sum = 0;
    for equation in equations {
        if equation.is_solvable(&ops) {
            println!("correct {equation:?}");
            correct_sum += equation.result;
        }
    }
    correct_sum
}

fn main() {
    let part = 2;
    let mut example = true;
    example = false;

    let input = if example {
        Cow::Borrowed(if part == 1 {
            PART1_EXAMPLE
        } else if !PART2_EXAMPLE.is_empty() {
            PART2_EXAMPLE
        } else {
            PART1_EXAMPLE
        })
    } else {
        Cow::Owned(fs::read_to_string("src/day07-input").unwrap())
    };

    let result = if part == 1 {
        part_one(&input)
    } else {
        part_two(&input)
    };

    println!("{}", result);
}

#[test]
fn test_answers() {
    let input = fs::read_to_string("src/day07-input").unwrap();
    assert_eq!(part_one(&input), 2941973819040);
    assert_eq!(part_two(&input), 249943041417600);
}
