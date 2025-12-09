use common::prelude::*;
use geo::{Contains, Coord, LineString, Polygon, Rect};
use glam::{uvec2, UVec2};

pub struct Day09_2025;

impl Solution for Day09_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve(input),
            PartNumber::Part2 => solve_part2(input),
        }
    }
}

solution!(
    Day09_2025,
    [
        example_part1(
            50,
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        ),
        solution_part1(Some(4790063600)),
        example_part2(
            24,
            "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
        ),
        solution_part2(Some(1516172795)),
    ]
);

#[test]
fn test_build_2025_09() {}

// -----

fn parse_tiles(input: &str) -> Vec<UVec2> {
    lines(input)
        .map(|s| {
            let (a, b) = s.split_once(',').unwrap();
            uvec2(a.parse().unwrap(), b.parse().unwrap())
        })
        .collect_vec()
}

fn solve(input: &str) -> i64 {
    let tiles = parse_tiles(input);

    tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let dx = (a.x as i64 - b.x as i64).abs() + 1;
            let dy = (a.y as i64 - b.y as i64).abs() + 1;
            dx * dy
        })
        .max()
        .unwrap()
}

fn solve_part2(input: &str) -> i64 {
    let tiles = parse_tiles(input);

    let allowed_polygon = Polygon::new(
        LineString::new(
            tiles
                .iter()
                .map(|t| Coord {
                    x: t.x as f64,
                    y: t.y as f64,
                })
                .collect_vec(),
        ),
        vec![],
    );

    use rayon::prelude::*;
    tiles
        .iter()
        .map(|p| Coord {
            x: p.x as f64,
            y: p.y as f64,
        })
        .tuple_combinations()
        .par_bridge()
        .filter_map(|(a, b)| {
            let min = Coord {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            };
            let max = Coord {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            };
            let rect = Rect::new(min, max);

            if !Contains::contains(&allowed_polygon, &rect) {
                None
            } else {
                let dx = (a.x as i64 - b.x as i64).abs() + 1;
                let dy = (a.y as i64 - b.y as i64).abs() + 1;
                Some(dx * dy)
            }
        })
        .max()
        .unwrap()
}
