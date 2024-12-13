use common::prelude::*;

pub struct Day04_2015;

impl Solution for Day04_2015 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => find_lowest_number(input, 5),
            PartNumber::Part2 => find_lowest_number(input, 6),
        }
    }
}

solution!(
    Day04_2015,
    [
        solution_part1(Some(254575)),
        example_part1(609043, "abcdef"),
        example_part1(1048970, "pqrstuv"),
        solution_part2(Some(1038736)),
    ]
);

#[test]
fn test_build_2015_04() {}

// -----

fn hash(input: &str, i: i64) -> String {
    let string = format!("{}{}", input, i);
    format!("{:x}", md5::compute(string))
}

fn find_lowest_number(input: &str, leading_zeros: usize) -> i64 {
    let prefix = "0".repeat(leading_zeros);
    (1..)
        .find_map(|i| hash(input, i).starts_with(&prefix).then_some(i))
        .unwrap()
}
