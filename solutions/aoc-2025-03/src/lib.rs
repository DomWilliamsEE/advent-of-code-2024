use common::prelude::*;
use std::cmp::Reverse;

pub struct Day03_2025;

impl Solution for Day03_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve(input, 2),
            PartNumber::Part2 => solve(input, 12),
        }
    }
}

solution!(
    Day03_2025,
    [
        example_part1(
            357,
            "987654321111111
811111111111119
234234234234278
818181911112111"
        ),
        solution_part1(Some(17535)),
        example_part2(
            3121910778619,
            "987654321111111
811111111111119
234234234234278
818181911112111"
        ),
        solution_part2(Some(173577199527257)),
    ]
);

#[test]
fn test_build_2025_03() {}

// -----

fn solve(input: &str, n: usize) -> i64 {
    let mut sum = 0;

    for (_bank_idx, bank) in lines(input).enumerate() {
        let mut res_so_far = 0;
        let mut start_from = 0;

        for i in 0..n {
            let end_to_have_enough_digits_left = bank.len() - (n - i - 1);
            let (next_idx, next_max_digit) = bank[start_from..end_to_have_enough_digits_left]
                .chars()
                .enumerate()
                .max_by_key(|&(i, c)| (c, Reverse(i))) // prefer earlier if same digit found
                .map(|(idx, c)| (idx + start_from, c))
                .unwrap();

            // println!(
            //     "bank {}, step {}: found digit {} at index {}",
            //     bank_idx, i, next_max_digit, next_idx
            // );

            res_so_far = res_so_far * 10 + (next_max_digit as u8 - b'0') as i64;
            start_from = next_idx + 1;
        }

        // println!("bank {}: result {}", bank_idx, res_so_far);
        sum += res_so_far;
    }

    sum
}

#[test]
fn test_examples() {
    assert_eq!(solve("987654321111111", 2), 98);
    assert_eq!(solve("234234234234278", 2), 78);
    assert_eq!(solve("818181911112111", 2), 92);
}

#[test]
fn test_examples_2() {
    assert_eq!(solve("987654321111111", 12), 987654321111);
    assert_eq!(solve("234234234234278", 12), 434234234278);
    assert_eq!(solve("818181911112111", 12), 888911112111);
}
