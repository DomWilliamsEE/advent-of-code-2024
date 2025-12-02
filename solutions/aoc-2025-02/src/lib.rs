use common::prelude::*;
use std::collections::HashSet;

pub struct Day02_2025;

impl Solution for Day02_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => find_invalids_part1(input).into_iter().sum::<u64>() as i64,
            PartNumber::Part2 => find_invalids_part2(input).into_iter().sum::<u64>() as i64,
        }
    }
}

const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

solution!(
    Day02_2025,
    [
        example_part1(1227775554, EXAMPLE),
        solution_part1(Some(5398419778)),
        example_part2(4174379265, EXAMPLE),
        solution_part2(Some(15704845910)),
    ]
);

#[test]
fn test_build_2025_02() {}

// -----

fn iter_ids(input: &str) -> impl Iterator<Item = u64> + '_ {
    input.trim().split(',').flat_map(|s| {
        let (a, b) = s.split_once('-').unwrap();
        let a: u64 = a.trim().parse().unwrap();
        let b: u64 = b.trim().parse().unwrap();
        a..=b
    })
}

fn find_invalids_part1(input: &str) -> HashSet<u64> {
    let mut invalids = HashSet::new();

    for id in iter_ids(input) {
        let id_str = id.to_string();

        let first_half = &id_str[..id_str.len() / 2];
        let second_half = &id_str[id_str.len() / 2..];
        if first_half == second_half {
            if invalids.insert(id) {
                println!("invalid: {} (repeat of {})", id, first_half);
            }
        }
    }

    invalids
}

fn find_invalids_part2(input: &str) -> HashSet<u64> {
    let mut invalids = HashSet::new();

    for id in iter_ids(input) {
        let id_str = id.to_string();

        for window_len in 1..id_str.len() {
            for window in id_str.as_bytes().windows(window_len) {
                let window = std::str::from_utf8(window).unwrap();

                if id_str.as_bytes().chunks(window_len).all_equal() {
                    if invalids.insert(id) {
                        println!("invalid: {} (repeat of {})", id, window);
                    }
                }
            }
        }
    }
    invalids
}
