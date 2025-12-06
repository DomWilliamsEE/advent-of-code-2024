use common::prelude::*;
use std::iter::once;

pub struct Day06_2025;

impl Solution for Day06_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve(input) as i64,
            PartNumber::Part2 => solve_char_grid(input) as i64,
        }
    }
}

solution!(
    Day06_2025,
    [
        example_part1(
            4277556,
            "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"
        ),
        solution_part1(Some(4583860641327)),
        example_part2(
            3263827,
            "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +"
        ),
        solution_part2(Some(11602774058280)),
    ]
);

#[test]
fn test_build_2025_06() {}

// -----

fn solve(input: &str) -> u64 {
    let mut lines = lines(input).collect_vec();
    let ops_line = lines.pop().unwrap();
    assert!(ops_line.contains("*"));

    let cells = lines
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let rows = lines.len() as u32;
    let cols = (cells.len() as u32) / rows;

    let ops = ops_line
        .chars()
        .filter(|s| !s.is_whitespace())
        .collect_vec();

    assert_eq!(ops.len() as u32, cols);

    let mut sum = 0;

    for col in 0..cols {
        let nums = cells
            .iter()
            .skip(col as usize)
            .step_by(cols as usize)
            .copied()
            .collect_vec();

        let op = ops[col as usize];
        let result: u64 = match op {
            '+' => nums.into_iter().sum(),
            '*' => nums.into_iter().product(),
            x => unreachable!("wat {x}"),
        };

        sum += result;
    }

    sum
}

fn solve_char_grid(input: &str) -> u64 {
    let mut lines = input.lines().collect_vec();
    let ops_line = lines.pop().unwrap();
    assert!(ops_line.contains("*"));

    let cells = lines
        .iter()
        .map(|s| once(' ').chain(s.chars()).collect_vec())
        .collect_vec();

    let rows = lines.len();

    let ops = ops_line.chars().collect_vec();

    // for row in &cells {
    //     println!("{:?}", row);
    // }

    let max_col = cells.iter().map(|r| r.len()).max().unwrap();

    let mut sum = 0;
    let mut start_col = max_col;
    while start_col > 0 {
        let mut nums = vec![];
        let mut cur_num = 0;

        for col in (0..start_col).rev() {
            for row in 0..rows {
                let char = cells
                    .get(row)
                    .and_then(|row| row.get(col))
                    .copied()
                    .unwrap_or(' ');

                if char.is_whitespace() {
                    continue;
                }

                let digit = char.to_digit(10).expect("not a digit");
                cur_num = cur_num * 10 + digit as u64;
            }

            if cur_num == 0 {
                let op = ops[col];

                let res = match op {
                    '+' => nums.iter().sum::<u64>(),
                    '*' => nums.iter().product::<u64>(),
                    x => unreachable!("wat {x}"),
                };

                println!("op at col {col}: {op}, {nums:?} = {res}");
                sum += res;
                start_col = col; // skip to next start

                nums.clear();
                continue;
            }

            nums.push(cur_num);
            cur_num = 0;
        }
    }

    sum
}
