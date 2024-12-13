use common::prelude::*;
use glam::IVec2;
use std::collections::HashSet;

pub struct Day03_2015;

impl Solution for Day03_2015 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => visit_houses(input.trim()),
            PartNumber::Part2 => visit_houses_with_robo(input.trim()),
        }
    }
}

solution!(
    Day03_2015,
    [
        solution_part1(Some(2572)),
        example_part1(2, ">"),
        example_part1(4, "^>v<"),
        example_part1(2, "^v^v^v^v^v"),
        solution_part2(Some(2631)),
        example_part2(3, "^v"),
        example_part2(3, "^>v<"),
        example_part2(11, "^v^v^v^v^v"),
    ]
);

#[test]
fn test_build_2015_03() {}

// -----

fn direction(c: char) -> IVec2 {
    match c {
        '>' => IVec2::new(0, 1),
        '<' => IVec2::new(0, -1),
        '^' => IVec2::new(1, 0),
        'v' => IVec2::new(-1, 0),
        _ => unreachable!("bad char {c}"),
    }
}

fn visit_houses(input: &str) -> i64 {
    let mut visited = HashSet::<IVec2>::default();
    visited.insert(IVec2::ZERO); // start

    input
        .chars()
        .fold((IVec2::ZERO, visited), |(pos, mut visited), c| {
            let next_pos = pos + direction(c);
            visited.insert(next_pos);
            (next_pos, visited)
        })
        .1
        .len() as i64
}

fn visit_houses_with_robo(input: &str) -> i64 {
    let mut visited = HashSet::<IVec2>::default();
    visited.insert(IVec2::ZERO); // start

    input
        .chars()
        .enumerate()
        .fold(
            ([IVec2::ZERO; 2], visited),
            |(mut pos, mut visited), (i, c)| {
                pos[i % 2] += direction(c);
                visited.insert(pos[i % 2]);

                (pos, visited)
            },
        )
        .1
        .len() as i64
}
