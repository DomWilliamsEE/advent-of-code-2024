use glam::{uvec2, IVec2, UVec2};
use itertools::Itertools;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Read;
use std::str::FromStr;

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
// leave blank if same as part1
const PART2_EXAMPLE: &str = "";

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

fn main() {
    let part = 2;
    let mut example = true;
    example = false;

    let input = if example {
        Cow::Borrowed(if part == 1 {
            PART1_EXAMPLE
        } else if !PART2_EXAMPLE.is_empty() {
            PART2_EXAMPLE
        } else {
            PART1_EXAMPLE
        })
    } else {
        Cow::Owned(fs::read_to_string("src/day08-input").unwrap())
    };

    let result = if part == 1 {
        part_one(&input)
    } else {
        part_two(&input)
    };

    println!("{}", result);
}

#[test]
fn test_answers() {
    assert_eq!(part_one(include_str!("day08-input")), 265);
    assert_eq!(part_two(include_str!("day08-input")), 962);
}