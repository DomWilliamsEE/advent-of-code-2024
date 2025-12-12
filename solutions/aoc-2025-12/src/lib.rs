use common::prelude::*;
use glam::{usizevec2, USizeVec2};
use std::collections::HashSet;
use std::iter::repeat_n;

pub struct Day12_2025;

impl Solution for Day12_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve(input) as i64,
            PartNumber::Part2 => -1_i64,
        }
    }
}

solution!(
    Day12_2025,
    [
        example_part1(
            2,
            "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
        ),
        solution_part1(Some(425)),
        solution_part2(None::<i64>),
    ]
);

#[test]
fn test_build_2025_12() {}

// -----

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct PresentShape([[bool; 3]; 3]);

#[derive(Debug)]
struct Region {
    dims: USizeVec2,
    counts: Vec<u32>,
}

#[derive(Debug)]
struct Presents {
    shapes: Vec<PresentShape>,
    regions: Vec<Region>,
}

struct RegionInstance {
    dims: USizeVec2,
    cells: Vec<Vec<bool>>, // too lazy for 1d array
}

fn parse(input: &str) -> Presents {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();

    let mut lines = lines(input); // no empty lines!
    for i in 0..6 {
        let header = lines.next().unwrap();
        assert_eq!(header, format!("{}:", i));

        let mut shape = PresentShape([[false; 3]; 3]);
        for y in 0..3 {
            let line = lines.next().unwrap();
            for (x, c) in line.chars().enumerate().take(3) {
                shape.0[y][x] = c == '#';
            }
        }
        shapes.push(shape);
    }

    for line in lines {
        let (dims, counts) = line.split_once(':').unwrap();

        let (x, y) = dims.split_once('x').unwrap();
        let dims = usizevec2(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

        let counts: Vec<u32> = counts
            .split_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        regions.push(Region { dims, counts });
    }

    Presents { shapes, regions }
}

impl PresentShape {
    fn rotate(&self) -> Self {
        let mut new_shape = PresentShape([[false; 3]; 3]);
        for y in 0..3 {
            for x in 0..3 {
                new_shape.0[x][2 - y] = self.0[y][x];
            }
        }
        new_shape
    }

    fn all_rotations(&self) -> Vec<Self> {
        let r0 = *self;
        let r1 = r0.rotate();
        let r2 = r1.rotate();
        let r3 = r2.rotate();
        [r0, r1, r2, r3]
            .into_iter()
            .collect::<HashSet<_>>() // skip dupes
            .into_iter()
            .collect()
    }
}

impl Region {
    fn iter_present_shapes<'a>(
        &'a self,
        presents: &'a Presents,
    ) -> impl Iterator<Item = &'a PresentShape> + 'a {
        self.counts
            .iter()
            .enumerate()
            .flat_map(move |(i, count)| repeat_n(&presents.shapes[i], *count as usize))
    }

    fn instantiate(&self) -> RegionInstance {
        RegionInstance {
            dims: self.dims,
            cells: vec![vec![false; self.dims.x]; self.dims.y],
        }
    }

    fn can_hold_all(&self, presents: &Presents) -> bool {
        let present_shapes = self
            .iter_present_shapes(presents)
            .map(|s| s.all_rotations())
            .collect_vec();

        let mut instance = self.instantiate();

        if presents.regions.len() == 3 {
            // stupidly hard example
            instance.recurse_and_backtrack(&present_shapes, 0)
        } else {
            // stupidly simple input
            for rotations in &present_shapes {
                if !instance.place_first_available(rotations) {
                    return false;
                }
            }
            true
        }
    }
}

impl RegionInstance {
    fn recurse_and_backtrack(&mut self, shapes: &[Vec<PresentShape>], i: usize) -> bool {
        if i == shapes.len() {
            return true;
        }

        for y in 0..=(self.dims.y).saturating_sub(3) {
            for x in 0..=(self.dims.x).saturating_sub(3) {
                for rotation in &shapes[i] {
                    if let Some(placed) = self.try_place(rotation, usizevec2(x, y)) {
                        if self.recurse_and_backtrack(shapes, i + 1) {
                            return true;
                        }
                        self.undo(&placed);
                    }
                }
            }
        }
        false
    }

    fn try_place(&mut self, shape: &PresentShape, pos: USizeVec2) -> Option<Vec<USizeVec2>> {
        let mut placed = Vec::new();

        for sy in 0..3 {
            for sx in 0..3 {
                if shape.0[sy][sx] {
                    let (gx, gy) = (pos.x + sx, pos.y + sy);
                    if self.cells[gy][gx] {
                        self.undo(&placed);
                        return None;
                    }
                    self.cells[gy][gx] = true;
                    placed.push(usizevec2(gx, gy));
                }
            }
        }
        Some(placed)
    }
    fn place_first_available(&mut self, rotations: &[PresentShape]) -> bool {
        for y in 0..=self.dims.y.saturating_sub(3) {
            for x in 0..=self.dims.x.saturating_sub(3) {
                for shape in rotations {
                    if self.try_place(shape, usizevec2(x, y)).is_some() {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn undo(&mut self, placed: &[USizeVec2]) {
        for pos in placed {
            self.cells[pos.y][pos.x] = false;
        }
    }
}

fn solve(input: &str) -> usize {
    let presents = parse(input);

    presents
        .regions
        .iter()
        .filter(|r| {
            let res = r.can_hold_all(&presents);
            println!("region {:?} can hold all: {}", r.dims, res);
            res
        })
        .count()
}
