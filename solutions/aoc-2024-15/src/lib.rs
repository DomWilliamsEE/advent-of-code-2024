use common::prelude::*;
use glam::{ivec2, uvec2, IVec2, UVec2};
use std::collections::HashSet;
use std::iter::{Extend, Iterator};

pub struct Day15_2024;

impl Solution for Day15_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => {
                let (mut world, insns) = World::parse(input);
                world.print_grid();

                for insn in insns {
                    println!("move {insn:?}");
                    world.move_robot(insn);
                    world.print_grid();
                }

                world
                    .iter_box_positions()
                    .map(|p| (p.x + 100 * p.y) as i64)
                    .sum::<i64>()
            }

            PartNumber::Part2 => {
                let (mut world, insns) = World::parse(input);
                world.make_wider();
                world.print_grid();

                for insn in insns {
                    println!("move {insn:?}");
                    world.move_robot(insn);
                    world.print_grid();
                }

                world
                    .iter_box_positions()
                    .map(|p| (p.x + 100 * p.y) as i64)
                    .sum::<i64>()
            }
        }
    }
}

solution!(
    Day15_2024,
    [
        solution_part1(Some(1495147)),
        example_part1(10092, PART1_EXAMPLE),
        example_part1(
            2028,
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
        ),
        solution_part2(Some(1524905)),
        /*        example_part2(
                    -1,
                    "#######
        #...#.#
        #.....#
        #..OO@#
        #..O..#
        #.....#
        #######

        <vv<<^^<<^^"
                ),
        */
        example_part2(9021, PART1_EXAMPLE),
    ]
);

const PART1_EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

#[test]
fn test_build_2024_15() {}

// -----

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Robot,
    SingleBox,
    BoxLeft,
    BoxRight,
}

struct World {
    grid: Vec<Cell>,
    dims: UVec2,
}

impl World {
    fn parse(s: &str) -> (Self, Vec<IVec2>) {
        let (map, instructions) = s.split_once("\n\n").unwrap();

        let grid = map
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| match c {
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                '@' => Cell::Robot,
                'O' => Cell::SingleBox,
                _ => unreachable!("bad char {c}"),
            })
            .collect_vec();

        let sz = (grid.len() as f32).sqrt() as usize;
        let dims = UVec2::splat(sz as u32);

        let insns = lines(instructions)
            .flat_map(|l| l.chars())
            .map(|c| match c {
                '^' => ivec2(0, -1),
                'v' => ivec2(0, 1),
                '<' => ivec2(-1, 0),
                '>' => ivec2(1, 0),
                _ => unreachable!("bad char {c}"),
            })
            .collect_vec();

        (World { grid, dims }, insns)
    }

    fn make_wider(&mut self) {
        self.dims.x *= 2;

        let new_map = self
            .grid
            .iter()
            .flat_map(|c| match c {
                Cell::Empty => [Cell::Empty; 2],
                Cell::Wall => [Cell::Wall; 2],
                Cell::Robot => [Cell::Robot, Cell::Empty],
                Cell::SingleBox => [Cell::BoxLeft, Cell::BoxRight],
                _ => unreachable!(),
            })
            .collect_vec();

        self.grid = new_map;
    }

    fn print_grid(&self) {
        let mut s = String::new();
        for y in 0..self.dims.y {
            for x in 0..self.dims.x {
                let cell = self.grid[(y * self.dims.x + x) as usize];
                let c = match cell {
                    Cell::Empty => '.',
                    Cell::Wall => '#',
                    Cell::Robot => '@',
                    Cell::SingleBox => 'O',
                    Cell::BoxLeft => '[',
                    Cell::BoxRight => ']',
                };
                s.push(c);
            }
            s.push('\n');
        }

        println!("{s}");
    }

    fn move_robot(&mut self, dir: IVec2) {
        assert_eq!(self.grid.iter().filter(|c| **c == Cell::Robot).count(), 1);

        let cur_idx = self.grid.iter().position(|c| *c == Cell::Robot).unwrap() as u32;
        let start_pos = uvec2(cur_idx % self.dims.x, cur_idx / self.dims.x).as_ivec2();

        let mut row = (1..)
            .map(|i| start_pos + (dir * i))
            .take_while(|p| {
                p.x >= 0 && p.x < self.dims.x as i32 && p.y >= 0 && p.y < self.dims.y as i32
            })
            .map(|p| {
                (
                    (p.x + p.y * self.dims.x as i32) as usize,
                    self.grid[(p.x + p.y * self.dims.x as i32) as usize],
                )
            })
            .take_while(|(_, c)| *c != Cell::Wall)
            .collect_vec();

        let Some(first_empty) = row.iter().position(|(_, c)| *c == Cell::Empty) else {
            return;
        };

        row.truncate(first_empty + 1);

        assert!(row
            .iter()
            .take(first_empty)
            .all(|(_, c)| matches!(c, Cell::SingleBox | Cell::BoxLeft | Cell::BoxRight)));

        let mut changes = vec![];

        changes.extend(
            row.iter()
                .copied()
                .tuple_windows()
                .map(|((_, a), (j, _))| (j, a)),
        );

        if dir.x == 0 && row.len() > 1 && !row.iter().any(|(_, c)| matches!(c, Cell::SingleBox)) {
            // vertical movement edge case with wide boxes
            let mut to_check = vec![start_pos + dir];
            let mut boxes_to_move = vec![];
            let mut checked = HashSet::new();

            while let Some(pos) = to_check.pop() {
                if !checked.insert(pos) {
                    continue;
                }

                let idx = (pos.x + pos.y * self.dims.x as i32) as usize;
                let c = self.grid[idx];

                match c {
                    Cell::BoxLeft => {
                        boxes_to_move.push(idx);
                        to_check.push(pos + ivec2(1, 0));
                        to_check.push(pos + dir);
                    }
                    Cell::BoxRight => {
                        boxes_to_move.push(idx);
                        to_check.push(pos + ivec2(-1, 0));
                        to_check.push(pos + dir);
                    }
                    Cell::Empty => continue,
                    Cell::Wall => return,
                    _ => unreachable!(),
                }
            }

            let boxes_to_move = boxes_to_move
                .into_iter()
                .filter_map(|i| {
                    if self.grid[i] == Cell::BoxLeft {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>()
                .into_iter()
                .sorted_by_key(|i| {
                    let y = *i as i32 / self.dims.x as i32;
                    y * -dir.y // top down
                })
                .collect_vec();

            let single_cells_to_move = boxes_to_move.into_iter().flat_map(|left| [left, left + 1]);

            for i in single_cells_to_move {
                let next = i + (dir.y * self.dims.x as i32) as usize;
                let next_cell = self.grid[next];
                if next_cell == Cell::Wall {
                    return;
                }

                changes.push((next, self.grid[i]));
                changes.push((i, Cell::Empty));
            }
        }

        changes.extend([
            // first becomes empty as robot moves out of it
            (cur_idx as usize, Cell::Empty),
            // second becomes robot
            (row[0].0, Cell::Robot),
        ]);

        for (i, c) in changes {
            self.grid[i] = c;
        }
    }

    fn iter_box_positions(&self) -> impl Iterator<Item = IVec2> + '_ {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, c)| matches!(**c, Cell::SingleBox | Cell::BoxLeft))
            .map(|(i, _)| ivec2(i as i32 % self.dims.x as i32, i as i32 / self.dims.x as i32))
    }
}
