use common::itertools::Itertools;
use common::{solution, PartNumber, Solution, SolutionInput};

pub struct Day02_2015;

impl Solution for Day02_2015 {
    fn solve(input: &str, part: PartNumber) -> i64 {
        match part {
            PartNumber::Part1 => input.lines().map(paper_requirement).sum(),
            PartNumber::Part2 => input.lines().map(ribbon_requirement).sum(),
        }
    }
}
solution!(
    Day02_2015,
    [
        (PartNumber::Part1, SolutionInput::FullInput, Some(1586300)),
        (PartNumber::Part1, SolutionInput::Example("2x3x4"), Some(58)),
        (PartNumber::Part2, SolutionInput::FullInput, Some(3737498)),
        (PartNumber::Part2, SolutionInput::Example("2x3x4"), Some(34)),
        (
            PartNumber::Part2,
            SolutionInput::Example("1x1x10"),
            Some(14)
        ),
    ]
);

#[test]
fn test_build_2015_02() {}

// -----

fn paper_requirement(line: &str) -> i64 {
    let dims = line
        .split('x')
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();

    let l = dims[0];
    let w = dims[1];
    let h = dims[2];

    let smallest = [l * w, w * h, h * l].into_iter().min().unwrap();

    ((2 * l * w + 2 * w * h + 2 * h * l) + smallest) as i64
}

fn ribbon_requirement(line: &str) -> i64 {
    let dims = line
        .split('x')
        .map(|s| s.parse::<u32>().unwrap())
        .collect_vec();

    let perimeters = dims.iter().combinations(2).map(|vec| {
        let a = *vec[0];
        let b = *vec[1];
        a + a + b + b
    });
    (perimeters.min().unwrap() + dims.iter().product::<u32>()) as i64
}
