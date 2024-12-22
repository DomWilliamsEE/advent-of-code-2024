extern crate core;

use common::prelude::*;
use glam::ivec2;
use std::collections::HashMap;
use std::iter::once;
use std::str::FromStr;

pub struct Day21_2024;

impl Solution for Day21_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => lines(input)
                .map(|line| {
                    let code = line.parse::<Code>().unwrap();
                    dew_it(&code, 2) * code.numeric
                })
                .sum::<i64>(),
            PartNumber::Part2 => lines(input)
                .map(|line| {
                    let code = line.parse::<Code>().unwrap();
                    dew_it(&code, 25) * code.numeric
                })
                .sum::<i64>(),
        }
    }
}

solution!(
    Day21_2024,
    [
        example_part1(
            126384,
            "029A
980A
179A
456A
379A"
        ),
        solution_part1(Some(215374)),
        solution_part2(Some(260586897262600)),
    ]
);

#[test]
fn test_build_2024_21() {}

// -----

#[derive(Debug)]
struct Code {
    chars: String,
    numeric: i64,
}

impl FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.ends_with('A'));
        Ok(Self {
            chars: s.to_string(),
            numeric: s[..s.len() - 1].parse().unwrap(),
        })
    }
}

fn dew_it(code: &Code, depth: usize) -> i64 {
    let mut cache = HashMap::new();
    let mut acc = 0;
    for (a, b) in once('A').chain(code.chars.chars()).tuple_windows() {
        let paths = find_numeric_paths(a, b);
        acc += paths
            .into_iter()
            .map(|s| recurse_directional(&s, depth, &mut cache))
            .min()
            .unwrap();
    }
    acc
}

fn find_numeric_paths(start: char, end: char) -> Vec<String> {
    let key_pos = |c| match c {
        '9' => ivec2(2, 0),
        '8' => ivec2(1, 0),
        '7' => ivec2(0, 0),
        '6' => ivec2(2, 1),
        '5' => ivec2(1, 1),
        '4' => ivec2(0, 1),
        '3' => ivec2(2, 2),
        '2' => ivec2(1, 2),
        '1' => ivec2(0, 2),
        '0' => ivec2(1, 3),
        'A' => ivec2(2, 3),
        _ => unreachable!(),
    };

    let start = key_pos(start);
    let end = key_pos(end);
    let delta = end - start;
    let [dx, dy] = delta.to_array();

    let mut output = Vec::new();

    let hor = match dx.signum() {
        -1 => "<".repeat(dx.abs() as usize),
        0 => String::new(),
        1 => ">".repeat(dx.abs() as usize),
        _ => unreachable!(),
    };

    let ver = match dy.signum() {
        -1 => "^".repeat(dy.abs() as usize),
        0 => String::new(),
        1 => "v".repeat(dy.abs() as usize),
        _ => unreachable!(),
    };

    if start.y != 3 || end.x != 0 {
        let mut path1 = String::new();
        path1.push_str(&hor);
        path1.push_str(&ver);
        path1.push('A');
        output.push(path1);
    }

    if start.x != 0 || end.y != 3 {
        let mut path2 = String::new();
        path2.push_str(&ver);
        path2.push_str(&hor);
        path2.push('A');
        output.push(path2);
    }

    output
}

fn recurse_directional(
    input: &str,
    depth: usize,
    cache: &mut HashMap<(String, usize), i64>,
) -> i64 {
    if let Some(cache) = cache.get(&(input.to_string(), depth)) {
        return *cache;
    }

    let mut acc = 0;
    for (a, b) in once('A').chain(input.chars()).tuple_windows() {
        let paths = find_directional_paths(a, b);

        if depth > 1 {
            acc += paths
                .into_iter()
                .map(|s| recurse_directional(&s, depth - 1, cache))
                .min()
                .unwrap();
        } else {
            acc += paths.into_iter().map(|s| s.len() as i64).min().unwrap();
        }
    }

    cache.insert((input.to_string(), depth), acc);
    acc
}

fn find_directional_paths(start: char, end: char) -> Vec<String> {
    let key_pos = |c| match c {
        '^' => ivec2(1, 0),
        '<' => ivec2(0, 1),
        'v' => ivec2(1, 1),
        '>' => ivec2(2, 1),
        'A' => ivec2(2, 0),
        _ => unreachable!(),
    };

    let start = key_pos(start);
    let end = key_pos(end);
    let delta = end - start;
    let [dx, dy] = delta.to_array();

    let mut output = Vec::new();

    let hor = match dx.signum() {
        -1 => "<".repeat(dx.abs() as usize),
        0 => String::new(),
        1 => ">".repeat(dx.abs() as usize),
        _ => unreachable!(),
    };

    let ver = match dy.signum() {
        -1 => "^".repeat(dy.abs() as usize),
        0 => String::new(),
        1 => "v".repeat(dy.abs() as usize),
        _ => unreachable!(),
    };

    if start.y != 0 || end.x != 0 {
        let mut path1 = String::new();
        path1.push_str(&hor);
        path1.push_str(&ver);
        path1.push('A');
        output.push(path1);
    }

    if start.x != 0 || end.y != 0 {
        let mut path2 = String::new();
        path2.push_str(&ver);
        path2.push_str(&hor);
        path2.push('A');
        output.push(path2);
    }

    output
}

#[test]
fn test_code() {
    let code: Code = "029A".parse().unwrap();
    assert_eq!(dew_it(&code, 2), 68);
}

#[test]
fn test_len() {
    assert_eq!(
        Day21_2024::solve("029A", PartNumber::Part1).into(),
        (68i64 * 29).into()
    );
}
