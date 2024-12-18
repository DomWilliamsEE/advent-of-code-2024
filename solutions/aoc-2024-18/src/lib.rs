use common::prelude::*;
use glam::{ivec2, UVec2};

pub struct Day18_2024;

impl Solution for Day18_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => {
                let mut space = parse_input(input);
                space.steps = 1024;
                space.find_path_len().unwrap().to_string()
            }
            PartNumber::Part2 => {
                let mut space = parse_input(input);
                let byte = space.find_first_blocking_byte().unwrap();
                format!("{},{}", byte.x, byte.y)
            }
        }
    }
}

solution!(
    Day18_2024,
    [solution_part1(Some(260)), solution_part2(Some("24,48")),]
);

const PART1_EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

#[test]
fn test_build_2024_18() {}

// -----

struct MemorySpace {
    byte_positions: Vec<UVec2>,
    /// inclusive
    sz: u32,
    steps: usize,
}

fn parse_input(input: &str) -> MemorySpace {
    let bytes = lines(input)
        .map(|line| line.split(',').map(|s| s.parse().unwrap()))
        .map(|mut xy| UVec2::new(xy.next().unwrap(), xy.next().unwrap()))
        .collect_vec();

    let sz = if bytes.len() == 25 { 6 } else { 70 };

    MemorySpace {
        byte_positions: bytes,
        sz,
        steps: 0,
    }
}

impl MemorySpace {
    fn find_path_len(&self) -> Option<i64> {
        let fallen_bytes = &self.byte_positions[..self.steps.min(self.byte_positions.len())];
        let start = UVec2::new(0, 0);
        let end = UVec2::splat(self.sz);

        pathfinding::prelude::bfs(
            &start,
            |pos| {
                let mut succ = vec![];

                for offset in [ivec2(-1, 0), ivec2(1, 0), ivec2(0, -1), ivec2(0, 1)] {
                    let new_pos = pos.as_ivec2() + offset;
                    if new_pos.x >= 0
                        && new_pos.x <= self.sz as i32
                        && new_pos.y >= 0
                        && new_pos.y <= self.sz as i32
                        && !fallen_bytes.contains(&new_pos.as_uvec2())
                    {
                        succ.push(new_pos.as_uvec2());
                    }
                }

                succ
            },
            |pos| *pos == end,
        )
        .map(|path| path.len() as i64 - 1)
    }

    fn find_first_blocking_byte(&mut self) -> Option<UVec2> {
        for (i, byte) in self.byte_positions.iter().copied().enumerate().rev() {
            self.steps = i;
            if self.find_path_len().is_some() {
                return Some(byte);
            }
        }

        None
    }
}

#[test]
fn part1_example() {
    let mut mem = parse_input(PART1_EXAMPLE);
    mem.steps = 12;
    assert_eq!(mem.find_path_len(), Some(22));
}

#[test]
fn part2_example() {
    let mut mem = parse_input(PART1_EXAMPLE);
    assert_eq!(mem.find_first_blocking_byte(), Some(UVec2::new(6, 1)));
}
