use common::prelude::*;

pub struct Day02_2015;

impl Solution for Day02_2015 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => input.lines().map(paper_requirement).sum::<i64>(),
            PartNumber::Part2 => input.lines().map(ribbon_requirement).sum::<i64>(),
        }
    }
}

solution!(
    Day02_2015,
    [
        solution_part1(Some(1586300)),
        example_part1(58, "2x3x4"),
        solution_part2(Some(3737498)),
        example_part2(34, "2x3x4"),
        example_part2(14, "1x1x10"),
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
