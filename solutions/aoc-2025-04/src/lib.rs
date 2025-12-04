use common::prelude::*;
use glam::{ivec2, uvec2, IVec2, UVec2};
use std::collections::HashSet;

pub struct Day04_2025;

impl Solution for Day04_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve_part1(input) as i64,
            PartNumber::Part2 => solve_part2(input) as i64,
        }
    }
}

solution!(
    Day04_2025,
    [
        example_part1(
            13,
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
        ),
        solution_part1(Some(1445)),
        example_part2(
            43,
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
        ),
        solution_part2(Some(8317)),
    ]
);

#[test]
fn test_build_2025_04() {}

// -----

struct Grid {
    cells: Vec<bool>,
    dims: UVec2,
    removed: Vec<bool>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let grid = lines(input)
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '@' => true,
                '.' => false,
                _ => unreachable!(),
            })
            .collect_vec();

        let sz = grid.len().isqrt();
        assert_eq!(sz * sz, grid.len(), "not square");

        let dims = UVec2::splat(sz as u32);
        Self {
            cells: grid,
            dims,
            removed: vec![false; sz * sz],
        }
    }

    fn get_cell(&self, pos: IVec2) -> Option<bool> {
        if pos.x < 0 || pos.y < 0 {
            None
        } else if pos.x >= self.dims.x as i32 || pos.y >= self.dims.y as i32 {
            None
        } else {
            let idx = (pos.y as u32 * self.dims.x + pos.x as u32) as usize;
            if self.removed[idx] {
                return None;
            }
            Some(self.cells[idx])
        }
    }

    fn print(&self, extras: &HashSet<UVec2>) {
        for y in 0..self.dims.y {
            for x in 0..self.dims.x {
                let idx = (y * self.dims.x + x) as usize;
                let c = if extras.contains(&uvec2(x, y)) {
                    'x'
                } else if self.removed[idx] {
                    ','
                } else {
                    if self.cells[idx] {
                        '@'
                    } else {
                        '.'
                    }
                };
                print!("{c}");
            }
            println!();
        }
    }

    fn find_accessible(&self) -> Vec<UVec2> {
        let mut accessible = Vec::new();
        for i in 0..self.cells.len() {
            let pos = uvec2(
                (i % self.dims.x as usize) as u32,
                (i / self.dims.x as usize) as u32,
            );
            let cell = self.get_cell(pos.as_ivec2());
            if !matches!(cell, Some(true)) {
                continue;
            }

            let offsets: [IVec2; 8] = [
                ivec2(0, 1),
                ivec2(1, 0),
                ivec2(0, -1),
                ivec2(-1, 0),
                ivec2(1, 1),
                ivec2(1, -1),
                ivec2(-1, 1),
                ivec2(-1, -1),
            ];

            let blocked_neighbours = offsets
                .iter()
                .filter(|off| {
                    let neighbour = pos.as_ivec2() + **off;
                    self.get_cell(neighbour).unwrap_or_default()
                })
                .count();

            if blocked_neighbours < 4 {
                accessible.push(pos);
            }
        }

        accessible
    }

    fn remove_cells(&mut self, cells: &Vec<UVec2>) {
        for pos in cells {
            let idx = (pos.y * self.dims.x + pos.x) as usize;
            self.removed[idx] = true;
        }
    }
}

fn solve_part1(input: &str) -> usize {
    let grid = Grid::new(input);
    let accessible = grid.find_accessible();
    grid.print(&accessible.iter().copied().collect());
    accessible.len()
}
fn solve_part2(input: &str) -> usize {
    let mut grid = Grid::new(input);

    let mut total_count = 0;

    loop {
        let accessible = grid.find_accessible();
        grid.print(&accessible.iter().copied().collect());
        println!("remove {}", accessible.len());

        if accessible.is_empty() {
            break;
        }

        total_count += accessible.len();
        grid.remove_cells(&accessible);
    }

    total_count
}
