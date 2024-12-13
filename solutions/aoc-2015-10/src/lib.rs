use common::prelude::*;
use std::iter::Iterator;

pub struct Day10_2015;

impl Solution for Day10_2015 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => apply(input, 40),
            PartNumber::Part2 => apply(input, 50),
        }
    }
}

solution!(
    Day10_2015,
    [solution_part1(Some(360154)), solution_part2(Some(5103798))]
);

#[test]
fn test_build_2015_10() {}

// -----

fn iter_sequences(input: &str) -> impl Iterator<Item = (char, usize)> {
    let mut result = Vec::new();
    let mut chars = input.chars();

    if let Some(first) = chars.next() {
        let mut curr = first;
        let mut count = 1;

        for c in chars {
            if c == curr {
                count += 1;
            } else {
                result.push((curr, count));
                curr = c;
                count = 1;
            }
        }
        result.push((curr, count));
    }

    result.into_iter()
}

fn apply(input: &str, n: usize) -> i64 {
    fn recurse(int: &str, this_depth: usize, max_depth: usize) -> usize {
        println!("at depth {this_depth}, string len {}", int.len());
        if this_depth == max_depth {
            return int.chars().count();
        }
        let mut string = String::new();

        for (char, n) in iter_sequences(int) {
            string.push_str(&n.to_string());
            string.push(char);
        }

        recurse(&string, this_depth + 1, max_depth)
    }

    let starting = input.trim();
    recurse(starting, 0, n) as i64
}

#[test]
fn test_example() {
    assert_eq!(apply("1", 1), 2);
    assert_eq!(apply("11", 1), 2);
    assert_eq!(apply("21", 1), 4);
    assert_eq!(apply("1211", 1), 6);
    assert_eq!(apply("111221", 1), 6);
}

#[test]
fn test_sequences() {
    assert_eq!(
        iter_sequences("222311").collect_vec(),
        vec![('2', 3), ('3', 1), ('1', 2)]
    );
}
