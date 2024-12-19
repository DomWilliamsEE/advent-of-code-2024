use common::prelude::*;
use pathfinding::prelude::{bfs, count_paths};
use std::default::Default;

pub struct Day19_2024;

impl Solution for Day19_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => parse_input(input).iter_possible_designs().count() as i64,
            PartNumber::Part2 => parse_input(input).count_all_possible_designs() as i64,
        }
    }
}

solution!(
    Day19_2024,
    [
        example_part1(6, PART1_EXAMPLE),
        solution_part1(Some(363)),
        example_part2(16, PART1_EXAMPLE),
        solution_part2(Some(642535800868438_i64)),
    ]
);

const PART1_EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[test]
fn test_build_2024_19() {}

// -----

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Towel(String);

#[derive(Debug, Clone)]
struct Design(String);

#[derive(Debug)]
struct Towels {
    available_towels: Vec<Towel>,
    desired: Vec<Design>,
}

fn parse_input(input: &str) -> Towels {
    let mut lines = lines(input);

    let available_towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| Towel(s.trim().to_string()))
        .collect_vec();
    let desired = lines.map(|s| Design(s.trim().to_string())).collect_vec();
    Towels {
        available_towels,
        desired,
    }
}

impl Towels {
    fn iter_possible_designs(&self) -> impl Iterator<Item = &Design> {
        self.desired
            .iter()
            .filter(|desired| desired.is_possible(&self.available_towels))
    }

    fn count_all_possible_designs(&self) -> usize {
        self.desired
            .iter()
            .map(|desired| desired.all_possible(&self.available_towels))
            .sum::<usize>()
    }
}

impl Towel {
    fn successors(&self, desired: &Design, available_towels: &[Towel]) -> Vec<Towel> {
        let mut succ = vec![];

        let remaining = &desired.0[self.0.len()..];

        if !self.0.chars().zip(desired.0.chars()).all(|(a, b)| a == b) {
            return succ;
        }

        for available in available_towels {
            if remaining.starts_with(&available.0) {
                succ.push(Towel(self.0.clone() + &available.0));
            }
        }

        succ
    }
}

impl Design {
    fn all_possible(&self, available_towels: &[Towel]) -> usize {
        count_paths(
            Towel::default(),
            |current| current.successors(self, available_towels),
            |towel| towel.0 == self.0,
        )
    }
    fn is_possible(&self, available_towels: &[Towel]) -> bool {
        bfs(
            &Towel::default(),
            |current| current.successors(self, available_towels),
            |towel| towel.0 == self.0,
        )
        .is_some()
    }
}
