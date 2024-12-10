use common::itertools::Itertools;
use common::{lines, solution, PartNumber, Solution, SolutionInput};
use glam::{ivec2, IVec2, UVec2};
use std::collections::HashMap;

pub struct Day10_2024;

impl Solution for Day10_2024 {
    fn solve(input: &str, part: PartNumber) -> i64 {
        match part {
            PartNumber::Part1 => {
                let map = Map::new(input);
                map.recurse_trailheads().len() as i64
            }
            PartNumber::Part2 => {
                let map = Map::new(input);
                map.recurse_trailheads().values().sum::<u32>() as i64
            }
        }
    }
}
solution!(
    Day10_2024,
    [
        (PartNumber::Part1, SolutionInput::FullInput, Some(587)),
        (
            PartNumber::Part1,
            SolutionInput::Example(PART1_EXAMPLE),
            Some(1)
        ),
        (
            PartNumber::Part1,
            SolutionInput::Example(
                "..90..9
        ...1.98
        ...2..7
        6543456
        765.987
        876....
        987...."
            ),
            Some(4)
        ),
        (
            PartNumber::Part1,
            SolutionInput::Example(
                "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"
            ),
            Some(36)
        ),
        (PartNumber::Part2, SolutionInput::FullInput, Some(1340)),
        (
            PartNumber::Part2,
            SolutionInput::Example(
                "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"
            ),
            Some(81)
        ),
    ]
);

const PART1_EXAMPLE: &str = "0123
1234
8765
9876";

#[test]
fn test_build_2024_10() {}

// -----

struct Map {
    dims: UVec2,
    grid: Vec<u32>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Trail {
    start_pos: UVec2,
    end_pos: UVec2,
}

const OFFSETS: [IVec2; 4] = [ivec2(0, 1), ivec2(1, 0), ivec2(0, -1), ivec2(-1, 0)];

impl Map {
    fn new(input: &str) -> Self {
        let grid = lines(input)
            .flat_map(|line| {
                line.chars().map(|c| {
                    if c == '.' {
                        255
                    } else {
                        c.to_digit(10).unwrap()
                    }
                })
            })
            .collect_vec();

        let sz = (grid.len() as f32).sqrt() as u32;

        Self {
            dims: UVec2::splat(sz),
            grid,
        }
    }

    /// (trail endpoints, count of routes)
    fn recurse_trailheads(&self) -> HashMap<Trail, u32> {
        fn recurse(
            pos: UVec2,
            current_val: u32,
            direction_to_take: IVec2,
            map: &Map,
            start_pos: UVec2,
            result: &mut HashMap<Trail, u32>,
        ) {
            assert!(current_val > 0);

            let next_pos = pos.as_ivec2() + direction_to_take;
            if next_pos.x >= 0
                && next_pos.x < map.dims.x as i32
                && next_pos.y >= 0
                && next_pos.y < map.dims.y as i32
            {
                // in bounds of grid
                let next_val = map.grid[(next_pos.x + next_pos.y * map.dims.x as i32) as usize];
                if next_val == current_val - 1 {
                    if next_val == 0 {
                        *result
                            .entry(Trail {
                                start_pos,
                                end_pos: next_pos.as_uvec2(),
                            })
                            .or_insert(0u32) += 1;
                    } else {
                        // recurse from next point
                        for offset in OFFSETS {
                            recurse(
                                next_pos.as_uvec2(),
                                next_val,
                                offset,
                                map,
                                start_pos,
                                result,
                            );
                        }
                    }
                }
            }
        }

        let mut trailhead_count = HashMap::new();
        for (i, val) in self
            .grid
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, c)| *c == 9)
        {
            let start_pos = UVec2::new(i as u32 % self.dims.x, i as u32 / self.dims.x);
            for offset in OFFSETS {
                recurse(
                    start_pos,
                    val,
                    offset,
                    self,
                    start_pos,
                    &mut trailhead_count,
                );
            }
        }

        trailhead_count
    }
}
