use common::prelude::*;

pub struct DayDD_YYYY;

impl Solution for DayDD_YYYY {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => todo!(),
            PartNumber::Part2 => todo!(),
        }
    }
}

solution!(
    DayDD_YYYY,
    [
        (PartNumber::Part1, SolutionInput::FullInput, None),
        (PartNumber::Part2, SolutionInput::FullInput, None),
    ]
);

#[test]
fn test_build_YYYY_DD() {}

// -----
