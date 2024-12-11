use common::itertools::Itertools;
use common::{lines, solution, PartNumber, Solution, SolutionInput};

pub struct Day05_2015;

impl Solution for Day05_2015 {
    fn solve(input: &str, part: PartNumber) -> i64 {
        match part {
            PartNumber::Part1 => lines(input).filter(|line| is_nice(line)).count() as i64,
            PartNumber::Part2 => lines(input).filter(|line| is_nice_v2(line)).count() as i64,
        }
    }
}
solution!(
    Day05_2015,
    [
        (PartNumber::Part1, SolutionInput::FullInput, Some(236)),
        (
            PartNumber::Part1,
            SolutionInput::Example("ugknbfddgicrmopn"),
            Some(1)
        ),
        (PartNumber::Part1, SolutionInput::Example("aaa"), Some(1)),
        (
            PartNumber::Part1,
            SolutionInput::Example("jchzalrnumimnmhp"),
            Some(0)
        ),
        (PartNumber::Part2, SolutionInput::FullInput, Some(51)),
        (
            PartNumber::Part2,
            SolutionInput::Example("qjhvhtzxzqqjkmpb"),
            Some(1)
        ),
        (PartNumber::Part2, SolutionInput::Example("xxyxx"), Some(1)),
        (
            PartNumber::Part2,
            SolutionInput::Example("uurcxstgmygtbstg"),
            Some(0)
        ),
        (
            PartNumber::Part2,
            SolutionInput::Example("ieodomkazucvgmuy"),
            Some(0)
        ),
    ]
);

#[test]
fn test_build_2015_05() {}

// -----

fn is_nice(input: &str) -> bool {
    let vowels = input.bytes().filter(|c| b"aeiou".contains(c)).count();
    let has_double = input.bytes().tuple_windows().any(|(a, b)| a == b);
    let has_blacklist = [b"ab", b"cd", b"pq", b"xy"].iter().any(|blacklisted| {
        input
            .as_bytes()
            .windows(2)
            .any(|window| window == *blacklisted)
    });

    !has_blacklist && (vowels >= 3 && has_double)
}

fn is_nice_v2(input: &str) -> bool {
    let has_nice_separated = input
        .bytes()
        .tuple_windows()
        .any(|(a, b, c)| a == c && a != b);

    has_nice_separated && has_nice_char_pair(input)
}

fn has_nice_char_pair(input: &str) -> bool {
    for i in 0..input.len() - 1 {
        let pair = &input[i..i + 2];
        let other_doesnt_overlap = input
            .match_indices(pair)
            .find(|(idx, _)| *idx > i)
            .is_some();

        if other_doesnt_overlap {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_nice_char_pair() {
        assert!(has_nice_char_pair("xyxy"));
        assert!(!has_nice_char_pair("aaa"));
        assert!(has_nice_char_pair("aabcdefgaa"));
        assert!(!has_nice_char_pair("ieodomkazucvgmuy"));
    }
}
