use common::prelude::*;
use glam::{ivec2, uvec2, IVec2, UVec2};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct Day14_2024;

impl Solution for Day14_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => safety_after(input, 100),
            PartNumber::Part2 => RobotGrid::new(input).simulate_until_christmas_tree(),
        }
    }
}

const PART1_EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

solution!(
    Day14_2024,
    [
        example_part1(12, PART1_EXAMPLE),
        solution_part1(Some(222901875)),
        solution_part2(Some(6243)),
    ]
);

#[test]
fn test_build_2024_14() {}

// -----

#[derive(Debug)]
struct Line {
    pos: IVec2,
    vel: IVec2,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ints = s
            .split(" ")
            .map(|s| {
                s.split("=")
                    .nth(1)
                    .unwrap()
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        Ok(Line {
            pos: ivec2(ints[0][0], ints[0][1]),
            vel: ivec2(ints[1][0], ints[1][1]),
        })
    }
}

fn safety_after(input: &str, seconds: u64) -> i64 {
    let grid = RobotGrid::new(input);

    let destinations = grid
        .robots
        .iter()
        .map(|line| {
            let dst = line.pos + (line.vel * seconds as i32);
            let mut res = ivec2(dst.x % grid.dims.x as i32, dst.y % grid.dims.y as i32);
            if res.x < 0 {
                res.x += grid.dims.x as i32;
            }
            if res.y < 0 {
                res.y += grid.dims.y as i32;
            }

            res
        })
        .collect_vec();

    let mut quadrant_count = [0i64; 4];
    let centre = (grid.dims / 2).as_ivec2();
    for dst in destinations {
        if dst.x == centre.x || dst.y == centre.y {
            continue;
        }

        let hor = dst.x < centre.x;
        let ver = dst.y < centre.y;

        let q = match (hor, ver) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        };

        quadrant_count[q] += 1;
    }

    quadrant_count.into_iter().product::<i64>()
}

struct RobotGrid {
    robots: Vec<Line>,
    dims: UVec2,
}

impl RobotGrid {
    fn new(input: &str) -> Self {
        let lines = lines(input)
            .map(|s| s.parse::<Line>().unwrap())
            .collect_vec();
        let dims = if lines.len() == 12 {
            uvec2(11, 7) // example
        } else {
            uvec2(101, 103)
        };

        Self {
            robots: lines,
            dims,
        }
    }

    fn print<'a>(&self, buf: &'a mut [u8]) -> (impl Display + 'a, bool) {
        buf.fill(b'.');

        for robot in &self.robots {
            let idx = (robot.pos.x + robot.pos.y * self.dims.x as i32) as usize;
            buf[idx] = b'X';
        }

        // 10 in a row
        let interesting = buf
            .chunks(self.dims.x as usize)
            .any(|line| line.windows(10).any(|w| w.iter().all(|&c| c == b'X')));

        struct Printer<'a>(&'a [u8], UVec2);
        impl Display for Printer<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let line = self.1.x as usize;
                for chunk in self.0.chunks(line) {
                    writeln!(f, "{}", std::str::from_utf8(chunk).unwrap())?;
                }
                Ok(())
            }
        }

        (Printer(buf, self.dims).to_string(), interesting)
    }

    fn simulate(&mut self) {
        let dims = self.dims.as_ivec2();
        self.robots.iter_mut().for_each(|robot| {
            robot.pos = (robot.pos + robot.vel) % dims;
            if robot.pos.x < 0 {
                robot.pos.x += dims.x;
            }
            if robot.pos.y < 0 {
                robot.pos.y += dims.y;
            }
        });
    }

    fn simulate_until_christmas_tree(&mut self) -> i64 {
        let mut buf = vec![0u8; (self.dims.x * self.dims.y) as usize];
        for i in 0.. {
            let (s, interesting) = self.print(&mut buf);

            // print!("{}\n{s}", self.second);

            if interesting {
                println!("{s}");
                return i;
            }
            self.simulate();
        }

        unreachable!()
    }
}
