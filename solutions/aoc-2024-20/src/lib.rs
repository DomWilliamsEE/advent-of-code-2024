use common::itertools::Either;
use common::prelude::*;
use glam::{ivec2, IVec2};
use pathfinding::prelude::{bfs, dijkstra};
use std::iter::empty;
use std::time::Instant;
use hashbrown::HashMap;

pub struct Day20_2024;

impl Solution for Day20_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => {
                let mut grid = parse_input(input);
                grid.count_cheats()
                    .filter_map(|(seconds_saved, count)| (seconds_saved >= 100).then_some(count))
                    .sum::<usize>() as i64
            }
            PartNumber::Part2 => -1_i64,
        }
    }
}

solution!(
    Day20_2024,
    [solution_part1(None::<i64>), solution_part2(None::<i64>),]
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
    // true if not blocked
    cells: Vec<bool>,

    endpoints: [IVec2; 2],

    normal_path_time: usize,
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
        .map(|(_, c)| c != '#')
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

#[derive(Debug, Clone)]
struct Cheat {
    start: IVec2,
    end: IVec2,
}

impl Grid {
    // fn iter_cheats(&self) -> impl Iterator<Item = IVec2> + '_ {

    fn normal_path_time(&self) -> usize {
        bfs(
            &self.endpoints[0],
            |pos| self.get_normal_successors(*pos),
            |pos| *pos == self.endpoints[1],
        )
        .unwrap()
        .len()
            - 1
    }

    fn cell_is_open(&self, pos: IVec2) -> bool {
        pos.x >= 0
            && pos.x < self.dims.x
            && pos.y >= 0
            && pos.y < self.dims.y
            && self.cells[pos.x as usize + pos.y as usize * self.dims.x as usize]
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

    fn get_cheat_successors(&self, pos: IVec2) -> impl Iterator<Item = (IVec2, IVec2)> + '_ {
        [ivec2(-1, 0), ivec2(1, 0), ivec2(0, -1), ivec2(0, 1)]
            .into_iter()
            .filter_map(move |dir| {
                let wall_pos = pos + dir;
                let back_on_track_pos = wall_pos + dir;

                if !self.cell_is_open(wall_pos) && self.cell_is_open(back_on_track_pos) {
                    return Some((back_on_track_pos, wall_pos));
                }

                None
            })
    }

    fn count_cheats(&self) -> impl Iterator<Item = (usize, usize)> {
        #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
        struct Node {
            pos: IVec2,
            time_so_far: usize,
            cheated: Option<[IVec2; 2]>,
        }

        let mut cheats_so_far = HashMap::new();

        loop {
            let start = Instant::now();
            let Some((path, cost)) = dijkstra(
                &Node {
                    pos: self.endpoints[0],
                    time_so_far: 0,
                    cheated: None,
                },
                |n| {
                    if n.time_so_far >=  self.normal_path_time {
                        return vec![];
                    }

                    let succ = self.get_normal_successors(n.pos).map(|succ| {
                        (
                            Node {
                                pos: succ,
                                time_so_far: n.time_so_far + 1,
                                cheated: n.cheated,
                            },
                            1,
                        )
                    });

                    let extra_cheats = if n.cheated.is_none() {
                        Either::Left(self.get_cheat_successors(n.pos).filter_map(
                            |(succ, wall_pos)| {
                                let cheat = [n.pos, succ];
                                (!cheats_so_far.contains_key(&cheat)).then(|| {
                                    (
                                        Node {
                                            pos: succ,
                                            time_so_far: n.time_so_far + 2,
                                            cheated: Some(cheat),
                                        },
                                        1,
                                    )
                                })
                            },
                        ))
                    } else {
                        Either::Right(empty())
                    };

                    extra_cheats.into_iter().chain(succ).collect_vec()
                },
                // |n| (n.pos.x - self.endpoints[1].x).abs() + (n.pos.y - self.endpoints[1].y).abs(),
                |n| n.pos == self.endpoints[1],
            ) else {
                println!("no path");
                break;
            };

            let elapsed = start.elapsed();

            let Some(cheat) = path.iter().find_map(|n| n.cheated) else {
                println!("no cheat");
                break;
            };

            let time = path.last().unwrap().time_so_far;
            let total_time = time;
            let saved_time = self.normal_path_time - total_time;
            println!("cheated with {cheat:?} to get total time {total_time}, saving {saved_time} ({:.2}s)", elapsed.as_secs_f32(),);

            // cheats_so_far.insert(cheat, saved_time);
            //     self.print(path.iter().map(|n| {
            //         (
            //             n.pos,
            //             n.cheated
            //                 .map(|c| c[1] == n.pos || c[0] == n.pos)
            //                 .unwrap_or(false),
            //         )
            //     }));

            cheats_so_far.insert(cheat, saved_time);
        }

        cheats_so_far
            .into_iter()
            .counts_by(|x| x.1)
            .into_iter()
            .sorted_by_key(|x| x.0)
    }

    fn print(&self, path: impl Iterator<Item = (IVec2, bool)>) {
        let mut grid = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let pos = ivec2(i as i32 % self.dims.x as i32, i as i32 / self.dims.x as i32);
                if pos == self.endpoints[0] {
                    'S'
                } else if pos == self.endpoints[1] {
                    'E'
                } else if *c {
                    '.'
                } else {
                    '#'
                }
            })
            .collect_vec();

        for (pos, cheat) in path {
            let mut c = &mut grid[(pos.y * self.dims.x + pos.x) as usize];
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
fn test_part1_input() {
    let grid = parse_input(PART1_EXAMPLE);
    println!("endpoints {:?}", grid.endpoints);
    assert_eq!(grid.normal_path_time, 84);
    // for i in 50..70 {
    //     println!("{i:02}: {}", grid.cheat(i));
    // }

    println!("grid");
    grid.print(empty());

    let cheats = grid.count_cheats().collect_vec();
    println!("{:?}", cheats);
    assert_eq!(
        cheats,
        vec![
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]
    )
}
