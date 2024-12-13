use common::prelude::*;
use glam::{uvec2, UVec2};
use std::str::FromStr;

pub struct Day06_2015;

impl Solution for Day06_2015 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => part_one(input),
            PartNumber::Part2 => part_two(input),
        }
    }
}

solution!(
    Day06_2015,
    [solution_part1(Some(400410)), solution_part2(Some(15343601))]
);

#[test]
fn test_build_2015_06() {}

// -----

#[derive(Debug, Copy, Clone)]
enum Setting {
    Toggle,
    Set(bool),
}

#[derive(Debug)]
struct Instruction {
    setting: Setting,
    range: [UVec2; 2],
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let prefixes = [
            ("turn on ", Setting::Set(true)),
            ("toggle ", Setting::Toggle),
            ("turn off ", Setting::Set(false)),
        ];
        let mut idx = 0;
        let setting = prefixes
            .iter()
            .find_map(|(prefix, setting)| {
                if line.starts_with(prefix) {
                    idx += prefix.len();
                    Some(*setting)
                } else {
                    None
                }
            })
            .ok_or("no setting?")?;

        let coords = (&line[idx..])
            .split(" through ")
            .map(|s| {
                let mut coords = s.trim().split(',').map(|s| s.parse::<u32>().unwrap());
                uvec2(coords.next().unwrap(), coords.next().unwrap())
            })
            .collect_vec();

        Ok(Instruction {
            setting,
            range: [coords[0], coords[1]],
        })
    }
}

#[test]
fn test_parsing() {
    let input = "turn on 0,0 through 999,999";
    dbg!(input.parse::<Instruction>().unwrap());
}

struct BoolGrid {
    dims: UVec2,
    data: Vec<bool>,
}

impl BoolGrid {
    fn new(dims: UVec2) -> Self {
        Self {
            dims,
            data: vec![false; (dims.x * dims.y) as usize],
        }
    }

    fn apply(&mut self, instruction: Instruction) {
        (instruction.range[0].y..=instruction.range[1].y)
            .cartesian_product(instruction.range[0].x..=instruction.range[1].x)
            .for_each(|(y, x)| {
                let idx = (y * self.dims.x + x) as usize;
                match instruction.setting {
                    Setting::Toggle => self.data[idx] = !self.data[idx],
                    Setting::Set(value) => self.data[idx] = value,
                }
            });
    }
}

fn part_one(input: &str) -> i64 {
    let mut grid = BoolGrid::new(uvec2(1000, 1000));
    for line in lines(input) {
        let instr: Instruction = line.parse().unwrap();
        grid.apply(instr);
    }

    grid.data.iter().filter(|b| **b).count() as i64
}

struct BrightnessGrid {
    dims: UVec2,
    data: Vec<u32>,
}

impl BrightnessGrid {
    fn new(dims: UVec2) -> Self {
        Self {
            dims,
            data: vec![0; (dims.x * dims.y) as usize],
        }
    }

    fn apply(&mut self, instruction: Instruction) {
        (instruction.range[0].y..=instruction.range[1].y)
            .cartesian_product(instruction.range[0].x..=instruction.range[1].x)
            .for_each(|(y, x)| {
                let idx = (y * self.dims.x + x) as usize;
                let val = &mut self.data[idx];
                match instruction.setting {
                    Setting::Toggle => *val += 2,
                    Setting::Set(true) => *val += 1,
                    Setting::Set(false) => *val = val.saturating_sub(1),
                }
            });
    }
}

fn part_two(input: &str) -> i64 {
    let mut grid = BrightnessGrid::new(uvec2(1000, 1000));
    for line in lines(input) {
        let instr: Instruction = line.parse().unwrap();
        grid.apply(instr);
    }

    grid.data.iter().sum::<u32>() as i64
}
