use common::prelude::*;
use glam::{ivec2, IVec2};
use pathfinding::prelude::astar_bag;
use std::collections::HashSet;
use std::iter::{once, IntoIterator, Iterator};

pub struct Day16_2024;

impl Solution for Day16_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve_part1(input),
            PartNumber::Part2 => solve_part2(input),
        }
    }
}

solution!(
    Day16_2024,
    [
        example_part1(7036, PART1_EXAMPLE),
        example_part1(
            11048,
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
        ),
        solution_part1(Some(93436)),
        example_part2(45, PART1_EXAMPLE),
        solution_part2(Some(486)),
    ]
);

const PART1_EXAMPLE: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

#[test]
fn test_build_2024_16() {}

// -----

#[derive(Copy, Clone, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn offset(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::new(0, -1),
            Direction::East => IVec2::new(1, 0),
            Direction::South => IVec2::new(0, 1),
            Direction::West => IVec2::new(-1, 0),
        }
    }

    fn neighbours(&self) -> [Direction; 2] {
        match self {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::East | Direction::West => [Direction::North, Direction::South],
        }
    }
}

struct Grid {
    width: i32,
    cells: Vec<Cell>,
}

impl Grid {
    fn parse(input: &str) -> (Self, [usize; 2]) {
        let (mut start, mut end) = (0, 0);
        let cells = lines(input)
            .flat_map(|line| line.chars())
            .enumerate()
            .map(|(i, c)| match c {
                '#' => Cell::Wall,
                'E' => {
                    end = i;
                    Cell::Empty
                }
                'S' => {
                    start = i;
                    Cell::Empty
                }
                '.' => Cell::Empty,
                _ => unreachable!(),
            })
            .collect_vec();
        assert_ne!(start, 0);
        assert_ne!(end, 0);

        let width = lines(input).next().unwrap().len() as i32;

        (Self { width, cells }, [start, end])
    }

    fn successors(&self, (n, cur_dir): (usize, Direction)) -> Vec<((usize, Direction), i64)> {
        let mut succ = vec![];

        let dirs = cur_dir
            .neighbours()
            .into_iter()
            .map(|d| (d, 1000_i64))
            .chain(once((cur_dir, 0)));
        for (d, extra_cost) in dirs {
            let candidate = ivec2(n as i32 % self.width, n as i32 / self.width) + d.offset();

            if candidate.x < 0
                || candidate.y < 0
                || candidate.x >= self.width
                || candidate.y >= self.width
            {
                continue;
            }
            let cell = self.cells[(candidate.y * self.width + candidate.x) as usize];
            if cell == Cell::Wall {
                continue;
            }

            succ.push((
                ((candidate.x + (candidate.y * self.width)) as usize, d),
                extra_cost + 1,
            ));
        }

        succ
    }
}

fn solve_part1(input: &str) -> i64 {
    let (grid, [start, end]) = Grid::parse(input);

    let path = pathfinding::directed::dijkstra::dijkstra(
        &(start, Direction::East),
        |&n| grid.successors(n),
        |(n, _)| *n == end,
    );

    path.unwrap().1
}

fn solve_part2(input: &str) -> i64 {
    let (grid, [start, end]) = Grid::parse(input);

    let paths = astar_bag(
        &(start, Direction::East),
        |&n| grid.successors(n),
        |n| {
            let n = ivec2(n.0 as i32 % grid.width, n.0 as i32 / grid.width);
            let end = ivec2(end as i32 % grid.width, end as i32 / grid.width);
            (n.distance_squared(end) as f32).sqrt() as i64
        },
        |n| n.0 == end,
    )
    .unwrap();

    paths
        .0
        .flat_map(|path| path.into_iter().map(|(n, _)| n))
        .collect::<HashSet<_>>()
        .len() as i64
}
