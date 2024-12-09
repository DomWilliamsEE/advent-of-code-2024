use common::itertools::Itertools;
use common::{solution, PartNumber, Solution, SolutionInput};
use glam::{uvec2, IVec2, UVec2};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Day08_2024;

impl Solution for Day08_2024 {
    fn solve(input: &str, part: PartNumber) -> i64 {
        match part {
            PartNumber::Part1 => part_one(input),
            PartNumber::Part2 => part_two(input),
        }
    }
}

const PART1_EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

solution!(
    Day08_2024,
    [
        (PartNumber::Part1, SolutionInput::FullInput, Some(265)),
        (
            PartNumber::Part1,
            SolutionInput::Example(PART1_EXAMPLE),
            Some(14),
        ),
        (PartNumber::Part2, SolutionInput::FullInput, Some(962)),
        (
            PartNumber::Part2,
            SolutionInput::Example(PART1_EXAMPLE),
            Some(34),
        ),
    ]
);

#[test]
fn test_build() {}

struct Map {
    cells: HashMap<char, Vec<UVec2>>,
    dims: UVec2,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut cells_per_freq = HashMap::<_, Vec<_>>::new();

        let mut dims = UVec2::ZERO;
        for (y, line) in input.lines().filter(|l| !l.is_empty()).enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    cells_per_freq
                        .entry(c)
                        .or_default()
                        .push(uvec2(x as u32, y as u32));
                }
                dims.x = x as u32 + 1;
            }

            dims.y = y as u32 + 1;
        }

        println!("{:?}", cells_per_freq);

        Ok(Map {
            cells: cells_per_freq,
            dims,
        })
    }
}

impl Map {
    fn is_in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as u32) < self.dims.x && (pos.y as u32) < self.dims.y
    }

    fn iter_all_antenodes<'a, I: Iterator<Item = IVec2> + 'a>(
        &'a self,
        candidates_fn: impl Fn([IVec2; 2]) -> I + 'a,
    ) -> impl Iterator<Item = UVec2> + 'a {
        self.cells
            .iter()
            .flat_map(|(freq, cells)| cells.iter().permutations(2))
            .flat_map(move |combo| {
                let a = combo[0].as_ivec2();
                let b = combo[1].as_ivec2();

                let candidates = candidates_fn([a, b]);

                // let grid = [a, b]
                //     .into_iter()
                //     .map(|antenna| (antenna.as_uvec2(), *freq))
                //     .chain(
                //         candidates
                //             .into_iter()
                //             .filter(|c| map.is_in_bounds(*c))
                //             .map(|antenna| (antenna.as_uvec2(), '#')),
                //     );
                // println!("grid for between {a:?} and {b:?}");
                // print_grid(grid, map.dims);

                // for c in &candidates {
                //     println!(
                //         "{freq:?} between {a:?} and {b:?}: {c:?} (valid {:?})",
                //         map.is_in_bounds(*c)
                //     );
                // }

                candidates
                    .filter(|c| self.is_in_bounds(*c))
                    .map(|c| c.as_uvec2())
            })
    }
}

fn print_grid(points: impl Iterator<Item = (UVec2, char)>, dims: UVec2) {
    let mut grid = vec!['.'; (dims.x * dims.y) as usize];

    for (pos, ch) in points {
        if pos.x < dims.x && pos.y < dims.y {
            grid[(pos.y * dims.x + pos.x) as usize] = ch;
        }
    }

    for y in 0..dims.y {
        let start = (y * dims.x) as usize;
        println!(
            "{}",
            grid[start..start + dims.x as usize]
                .iter()
                .collect::<String>()
        );
    }
}

fn part_one(input: &str) -> i64 {
    let map: Map = input.parse().unwrap();

    let uniques = map
        .iter_all_antenodes(|[a, b]| {
            let vec = b - a;
            [a - vec, b + vec].into_iter()
        })
        .collect::<HashSet<_>>();

    let end_grid = map
        .cells
        .iter()
        .flat_map(|(freq, cells)| cells.iter().map(|c| (*c, *freq)))
        .chain(uniques.iter().map(|c| (*c, '#')));
    // println!("END");
    print_grid(end_grid, map.dims);
    uniques.len() as i64
}

fn part_two(input: &str) -> i64 {
    let map: Map = input.parse().unwrap();

    let uniques = map
        .iter_all_antenodes(|[a, b]| {
            let vec = b - a;
            (0..1000i32)
                .map(move |i| a - vec * i)
                .take_while(|c| map.is_in_bounds(*c))
                .chain(
                    (0..1000i32)
                        .map(move |i| b + vec * i)
                        .take_while(|c| map.is_in_bounds(*c)),
                )
        })
        .collect::<HashSet<_>>();

    let end_grid = map
        .cells
        .iter()
        .flat_map(|(freq, cells)| cells.iter().map(|c| (*c, *freq)))
        .chain(uniques.iter().map(|c| (*c, '#')));
    // println!("END");
    print_grid(end_grid, map.dims);
    uniques.len() as i64
}
