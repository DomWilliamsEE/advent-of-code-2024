use common::prelude::*;

pub struct Day25_2024;

impl Solution for Day25_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve(input),
            PartNumber::Part2 => -1_i64,
        }
    }
}

solution!(
    Day25_2024,
    [
        example_part1(
            3,
            "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####"
        ),
        solution_part1(Some(3090)),
        solution_part2(None::<i64>),
    ]
);

#[test]
fn test_build_2024_25() {}

// -----

fn solve(input: &str) -> i64 {
    let mut keys = vec![];
    let mut locks = vec![];
    for lines in lines(input).chunks(7).into_iter() {
        let mut lines = lines.collect_vec();
        let downwards = lines[0].chars().all(|c| c == '#');

        if !downwards {
            lines.reverse();
        }

        let mut pins = [0i8; 5];

        assert_eq!(lines[0].len(), 5);
        for (x, pin) in pins.iter_mut().enumerate() {
            *pin = (1..6)
                .take_while(|y| lines[*y].as_bytes()[x] == b'#')
                .count() as i8;
        }

        if downwards {
            locks.push(pins);
        } else {
            keys.push(pins);
        }
    }

    keys.iter()
        .cartesian_product(locks.iter())
        .filter(|(key, lock)| key.iter().zip(lock.iter()).all(|(a, b)| (*a + *b) <= 5))
        .count() as i64
}
