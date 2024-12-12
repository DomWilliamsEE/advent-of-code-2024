use common::itertools::Itertools;
use common::{example_part1, example_part2, lines, solution, PartNumber, Solution, SolutionInput};
use geo::{Area, BooleanOps, Euclidean, Length, MultiPolygon, Polygon, Rect};
use glam::{ivec2, uvec2, IVec2, UVec2};
use std::collections::HashSet;

pub struct Day12_2024;

impl Solution for Day12_2024 {
    fn solve(input: &str, part: PartNumber) -> i64 {
        match part {
            PartNumber::Part1 => Grid::new(input).calculate_fence_price::<ModernPricing>(),
            PartNumber::Part2 => Grid::new(input).calculate_fence_price::<BulkPricing>(),
        }
    }
}
solution!(
    Day12_2024,
    [
        (PartNumber::Part1, SolutionInput::FullInput, Some(1485656)),
        example_part1(
            140,
            "AAAA
BBCD
BBCC
EEEC"
        ),
        example_part1(
            772,
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
        ),
        example_part1(
            1930,
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
        ),
        (PartNumber::Part2, SolutionInput::FullInput, Some(899196)),
        example_part2(
            80,
            "AAAA
BBCD
BBCC
EEEC"
        ),
        example_part2(
            436,
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
        ),
        example_part2(
            1206,
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
        ),
    ]
);

#[test]
fn test_build_2024_12() {}

// -----

struct Grid {
    dims: UVec2,
    cells: Vec<char>,
}

trait Pricing {
    fn region_price(region: &Polygon<f32>) -> i64;
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells = lines(input).flat_map(|line| line.chars()).collect_vec();

        let sz = (cells.len() as f32).sqrt() as u32;

        Self {
            dims: UVec2::new(sz, sz),
            cells,
        }
    }

    fn index_to_pos(&self, idx: usize) -> UVec2 {
        let x = idx % self.dims.x as usize;
        let y = idx / self.dims.x as usize;
        uvec2(x as u32, y as u32)
    }

    fn pos_to_index(&self, pos: UVec2) -> usize {
        let x = pos.x as usize;
        let y = pos.y as usize;
        y * self.dims.x as usize + x
    }

    fn flood_fill(&self, start: UVec2, visited: &mut HashSet<UVec2>) -> (char, Vec<UVec2>) {
        let mut frontier = vec![start];
        let c = self.cells[self.pos_to_index(start)];

        let mut region = vec![];
        while let Some(pos) = frontier.pop() {
            if visited.contains(&pos) || self.cells[self.pos_to_index(pos)] != c {
                continue;
            }

            visited.insert(pos);
            region.push(pos);

            const OFFSETS: [IVec2; 4] = [ivec2(0, 1), ivec2(1, 0), ivec2(0, -1), ivec2(-1, 0)];

            for offset in OFFSETS {
                let next_pos = pos.as_ivec2() + offset;
                if next_pos.x >= 0
                    && next_pos.x < self.dims.x as i32
                    && next_pos.y >= 0
                    && next_pos.y < self.dims.y as i32
                {
                    frontier.push(next_pos.as_uvec2());
                }
            }
        }

        (c, region)
    }

    fn calculate_fence_price<P: Pricing>(&self) -> i64 {
        let mut visited = HashSet::with_capacity(self.dims.x.pow(2) as usize);
        let mut regions = vec![];

        for (i, _) in self.cells.iter().copied().enumerate() {
            let pos = self.index_to_pos(i);
            if visited.contains(&pos) {
                continue;
            }

            let (c, region) = self.flood_fill(pos, &mut visited);

            // println!("{c:?}: {:?}", region);
            regions.push((c, region));
        }

        let polys = regions
            .into_iter()
            .map(|(c, region)| {
                let mut plots = region.into_iter().map(|pos| {
                    Rect::new(
                        (pos.x as f32 - 0.5, pos.y as f32 - 0.5),
                        (pos.x as f32 + 0.5, pos.y as f32 + 0.5),
                    )
                    .to_polygon()
                });

                let merged = MultiPolygon::new(vec![plots.next().unwrap()]);
                let mut merged = plots.fold(merged, |merged, plot| merged.union(&plot));

                assert_eq!(merged.0.len(), 1);
                (c, merged.0.remove(0))
            })
            .collect_vec();

        let mut sum = 0;
        for (_, poly) in polys {
            sum += P::region_price(&poly);
        }

        sum
    }
}

struct ModernPricing;

impl Pricing for ModernPricing {
    fn region_price(region: &Polygon<f32>) -> i64 {
        let perimeter = region.exterior().length::<Euclidean>()
            + region
                .interiors()
                .iter()
                .map(|p| p.length::<Euclidean>())
                .sum::<f32>();
        perimeter as i64 * region.unsigned_area() as i64
    }
}

struct BulkPricing;

impl Pricing for BulkPricing {
    fn region_price(region: &Polygon<f32>) -> i64 {
        let side_count = region.exterior().lines().len()
            + region
                .interiors()
                .iter()
                .map(|p| p.lines().count())
                .sum::<usize>();
        side_count as i64 * region.unsigned_area() as i64
    }
}

#[test]
fn test_grid() {
    let grid = Grid::new(
        "AAAA
BBCD
BBCC
EEEC",
    );

    println!("{:?} {:?}", grid.dims, grid.cells);

    dbg!(grid.calculate_fence_price::<ModernPricing>());
}
