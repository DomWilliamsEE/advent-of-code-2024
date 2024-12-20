use common::prelude::*;
use glam::{ivec2, IVec2};
use pathfinding::prelude::bfs;
use std::iter::empty;

pub struct Day20_2024;

impl Solution for Day20_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => {
                let grid = parse_input(input);
                grid.count_cheats_saving_at_least(2, 100)
            }
            PartNumber::Part2 => {
                let grid = parse_input(input);
                grid.count_cheats_saving_at_least(20, 100)
            }
        }
    }
}

solution!(
    Day20_2024,
    [solution_part1(Some(1296)), solution_part2(Some(977665)),]
);

const PART1_EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[test]
fn test_build_2024_20() {}

// -----

struct Grid {
    dims: IVec2,
    cells: Vec<Cell>,

    endpoints: [IVec2; 2],

    normal_path_time: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Space { dist_from_end: u32 },
}

fn parse_input(input: &str) -> Grid {
    let [mut start, mut end] = [IVec2::NEG_ONE; 2];
    let dims = IVec2::splat(input.lines().next().unwrap().len() as i32);
    let cells = lines(input)
        .flat_map(|line| line.chars())
        .enumerate()
        .inspect(|(i, c)| match c {
            'S' => {
                start = ivec2(*i as i32 % dims.x, *i as i32 / dims.x);
            }
            'E' => {
                end = ivec2(*i as i32 % dims.x, *i as i32 / dims.x);
            }
            _ => {}
        })
        .map(|(_, c)| match c {
            '#' => Cell::Wall,
            '.' | 'S' | 'E' => Cell::Space { dist_from_end: 0 },
            _ => unreachable!(),
        })
        .collect_vec();

    let mut grid = Grid {
        dims,
        cells,
        endpoints: [start, end],
        normal_path_time: 0,
    };
    grid.normal_path_time = grid.normal_path_time();
    grid
}

impl Grid {
    fn normal_path_time(&mut self) -> usize {
        let path = bfs(
            &self.endpoints[0],
            |pos| self.get_normal_successors(*pos),
            |pos| *pos == self.endpoints[1],
        )
        .unwrap();
        for (i, pos) in path.iter().enumerate() {
            let cell = &mut self.cells[pos.y as usize * self.dims.x as usize + pos.x as usize];
            assert!(matches!(cell, Cell::Space { .. }));
            *cell = Cell::Space {
                dist_from_end: (path.len() - i) as u32,
            };
        }

        path.len() - 1
    }

    fn cell(&self, pos: IVec2) -> Option<Cell> {
        (pos.x >= 0 && pos.x < self.dims.x && pos.y >= 0 && pos.y < self.dims.y)
            .then(|| self.cells[pos.x as usize + pos.y as usize * self.dims.x as usize])
    }

    fn cell_is_open(&self, pos: IVec2) -> bool {
        self.cell(pos)
            .map(|c| matches!(c, Cell::Space { .. }))
            .unwrap_or(false)
    }

    fn get_normal_successors(&self, pos: IVec2) -> impl Iterator<Item = IVec2> + '_ {
        [ivec2(-1, 0), ivec2(1, 0), ivec2(0, -1), ivec2(0, 1)]
            .into_iter()
            .filter_map(move |dir| {
                let pos = pos + dir;
                if self.cell_is_open(pos) {
                    return Some(pos);
                }
                None
            })
    }

    fn count_cheats_saving_at_least(&self, cheat_dist: i32, min_saving: i32) -> i64 {
        let mut count = 0;

        for (i, cell) in self.cells.iter().enumerate() {
            let start_pos = match cell {
                Cell::Space { dist_from_end } => *dist_from_end,
                _ => continue,
            };
            let pos = ivec2(i as i32 % self.dims.x, i as i32 / self.dims.x);

            for dx in -cheat_dist..=cheat_dist {
                let left = cheat_dist - dx.abs();
                for dy in -left..=left {
                    let new_pos = pos + ivec2(dx, dy);
                    let manhatten_dist = dx.abs() + dy.abs();

                    let shortcut_dist = match self.cell(new_pos) {
                        Some(Cell::Space { dist_from_end }) => dist_from_end,
                        _ => continue,
                    };

                    if start_pos as i32 - shortcut_dist as i32 - manhatten_dist >= min_saving {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn print(&self, path: impl Iterator<Item = (IVec2, bool)>) {
        let mut grid = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let pos = ivec2(i as i32 % self.dims.x, i as i32 / self.dims.x);
                if pos == self.endpoints[0] {
                    'S'
                } else if pos == self.endpoints[1] {
                    'E'
                } else {
                    match c {
                        Cell::Wall => '#',
                        Cell::Space { .. } => {
                            // (b'0' + (*dist_from_end as u8) % 10) as char
                            '.'
                        }
                    }
                }
            })
            .collect_vec();

        for (pos, cheat) in path {
            let c = &mut grid[(pos.y * self.dims.x + pos.x) as usize];
            let new_c = if cheat { 'C' } else { 'o' };

            if new_c != 'o' || !c.is_ascii_uppercase() {
                *c = new_c;
            }
        }

        for y in 0..self.dims.y {
            let start = (y * self.dims.x) as usize;
            println!(
                "{}",
                grid[start..start + self.dims.x as usize]
                    .into_iter()
                    .join("")
            );
        }
    }
}

#[test]
fn test_example() {
    let grid = parse_input(PART1_EXAMPLE);
    println!("endpoints {:?}", grid.endpoints);
    assert_eq!(grid.normal_path_time, 84);

    grid.print(empty());
    assert_eq!(grid.count_cheats_saving_at_least(2, 64), 1);
    assert_eq!(grid.count_cheats_saving_at_least(2, 20), 5);
    assert_eq!(grid.count_cheats_saving_at_least(2, 10), 10);

    assert_eq!(grid.count_cheats_saving_at_least(20, 76), 3);
    assert_eq!(grid.count_cheats_saving_at_least(20, 74), 7);
}
