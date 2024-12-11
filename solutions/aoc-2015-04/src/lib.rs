use common::{solution, PartNumber, Solution, SolutionInput};

pub struct Day04_2015;

impl Solution for Day04_2015 {
    fn solve(input: &str, part: PartNumber) -> i64 {
        match part {
            PartNumber::Part1 => find_lowest_number(input, 5),
            PartNumber::Part2 => find_lowest_number(input, 6),
        }
    }
}
solution!(
    Day04_2015,
    [
        (PartNumber::Part1, SolutionInput::FullInput, Some(254575)),
        (
            PartNumber::Part1,
            SolutionInput::Example("abcdef"),
            Some(609043)
        ),
        (
            PartNumber::Part1,
            SolutionInput::Example("pqrstuv"),
            Some(1048970)
        ),
        (PartNumber::Part2, SolutionInput::FullInput, Some(1038736)),
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
