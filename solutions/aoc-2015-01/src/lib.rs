use common::{solution, PartNumber, Solution, SolutionInput};

pub struct Day01_2015;

impl Solution for Day01_2015 {
    fn solve(input: &str, part: PartNumber) -> i64 {
        match part {
            PartNumber::Part1 => find_destination_floor(input.trim()),
            PartNumber::Part2 => find_basement_pos(input.trim()),
        }
    }
}
solution!(
    Day01_2015,
    [
        (PartNumber::Part1, SolutionInput::FullInput, Some(74)),
        (
            PartNumber::Part1,
            SolutionInput::Example("(()(()("),
            Some(3)
        ),
        (PartNumber::Part2, SolutionInput::FullInput, Some(1795)),
        (PartNumber::Part2, SolutionInput::Example("()())"), Some(5)),
    ]
);

#[test]
fn test_build_2015_01() {}

// -----

fn find_destination_floor(line: &str) -> i64 {
    line.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!("bad char {c}"),
        })
        .sum()
}

fn find_basement_pos(line: &str) -> i64 {
    line.chars()
        .scan(0, |floor, c| {
            *floor += match c {
                '(' => 1,
                ')' => -1,
                _ => unreachable!("bad char {c}"),
            };

            Some(*floor)
        })
        .enumerate()
        .find_map(|(i, floor)| (floor == -1).then_some(i as i64 + 1))
        .expect("basement not found")
}
