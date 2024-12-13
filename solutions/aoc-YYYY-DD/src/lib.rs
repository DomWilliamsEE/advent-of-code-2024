use common::prelude::*;

pub struct DayDD_YYYY;

impl Solution for DayDD_YYYY {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => -1_i64,
            PartNumber::Part2 => -1_i64,
        }
    }
}

solution!(
    DayDD_YYYY,
    [
        solution_part1(None::<i64>),

        solution_part2(None::<i64>),
    ]
);

#[test]
fn test_build_YYYY_DD() {}

// -----
