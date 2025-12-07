use common::prelude::*;
use std::collections::HashMap;

pub struct Day07_2025;

impl Solution for Day07_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve_part1(input) as i64,
            PartNumber::Part2 => solve_part2(input) as i64,
        }
    }
}

solution!(
    Day07_2025,
    [
        example_part1(
            21,
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        ),
        solution_part1(Some(1626)),
        example_part2(
            40,
            ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        ),
        solution_part2(Some(48989920237096)),
    ]
);

#[test]
fn test_build_2025_07() {}

// -----

fn solve_part1(input: &str) -> usize {
    let lines = lines(input).collect_vec();
    let line_len = lines[0].len();

    let mut beams = vec![false; line_len];
    let start_idx = lines[0].chars().position(|c| c == 'S').unwrap();
    beams[start_idx] = true;

    let mut split_count = 0;
    for line in &lines {
        let splits = line.chars().positions(|c| c == '^');
        for split in splits {
            if !beams[split] {
                continue;
            }

            beams[split] = false;
            if split > 0 {
                beams[split - 1] = true;
            }
            if split < line_len {
                beams[split + 1] = true;
            }

            split_count += 1;
        }

        let mut new_line = line.to_string();
        for (i, b) in beams.iter().enumerate() {
            if *b {
                new_line.replace_range(i..=i, "|");
            }
        }
        let beams = beams.iter().filter(|&&b| b).count();
        println!("{new_line} {beams} {split_count}");
    }

    split_count
}

fn solve_part2(input: &str) -> usize {
    let lines = lines(input).collect_vec();
    let start_idx = lines[0].chars().position(|c| c == 'S').unwrap();

    fn recurse(
        lines: &[&str],
        current_line: usize,
        beam_index: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        // println!("recurse line {current_line} with beam {beam_index}");
        if current_line >= lines.len() {
            return 1;
        }

        if let Some(cached) = cache.get(&(current_line, beam_index)) {
            return *cached;
        }

        let line = lines[current_line];

        let mut total = 0;
        if line.chars().nth(beam_index).unwrap() == '^' {
            for new_split in [beam_index.checked_sub(1), Some(beam_index + 1)] {
                let new_beam = match new_split {
                    Some(s) if s < line.len() => s,
                    _ => continue,
                };

                total += recurse(lines, current_line + 1, new_beam, cache);
            }
        } else {
            total += recurse(lines, current_line + 1, beam_index, cache);
        }

        cache.insert((current_line, beam_index), total);
        total
    }

    recurse(&lines, 1, start_idx, &mut HashMap::new())
}
