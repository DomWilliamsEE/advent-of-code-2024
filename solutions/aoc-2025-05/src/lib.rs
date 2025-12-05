use common::prelude::*;
use std::ops::RangeInclusive;

pub struct Day05_2025;

impl Solution for Day05_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve(input) as i64,
            PartNumber::Part2 => solve_part2(input) as i64,
        }
    }
}

solution!(
    Day05_2025,
    [
        example_part1(
            3,
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        ),
        solution_part1(Some(509)),
        example_part2(
            14,
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        ),
        solution_part2(Some(336790092076620)),
    ]
);

#[test]
fn test_build_2025_05() {}

// -----

fn parse_ranges<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<RangeInclusive<u64>> {
    input
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let (a, b) = s.trim().split_once('-').unwrap();
            a.parse::<u64>().unwrap()..=b.parse::<u64>().unwrap()
        })
        .collect_vec()
}

fn solve(input: &str) -> usize {
    let mut lines = input.lines();

    let ranges = parse_ranges(&mut lines);

    let ids = lines
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let fresh = ids.iter().filter(|i| ranges.iter().any(|r| r.contains(i)));

    fresh.count()
}

fn solve_part2(input: &str) -> usize {
    let mut lines = input.lines();

    let mut ranges = parse_ranges(&mut lines);

    // merge overlapping ranges
    ranges.sort_unstable_by_key(|r| *r.start());
    let mut merged_ranges = Vec::<RangeInclusive<u64>>::new();
    for r in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if *r.start() <= *last.end() + 1 {
                *last = *last.start()..=*last.end().max(r.end());
                continue;
            }
        }
        merged_ranges.push(r);
    }

    let range_counts = merged_ranges
        .iter()
        .map(|r| (r.end() - r.start() + 1) as usize)
        .sum::<usize>();

    range_counts
}
