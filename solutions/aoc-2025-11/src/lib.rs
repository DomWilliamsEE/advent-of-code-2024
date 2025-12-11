use common::prelude::*;
use std::collections::HashMap;

pub struct Day11_2025;

impl Solution for Day11_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve(input),
            PartNumber::Part2 => solve_part2(input),
        }
    }
}

solution!(
    Day11_2025,
    [
        example_part1(
            5,
            "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
        ),
        solution_part1(Some(506)),
        example_part2(
            2,
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
        ),
        solution_part2(Some(385912350172800)),
    ]
);

#[test]
fn test_build_2025_11() {}

// -----

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    lines(input)
        .map(|s| {
            let (node, rest) = s.split_once(": ").unwrap();
            let dests = rest.split(' ').map(|s| s.to_string()).collect::<Vec<_>>();
            (node.to_string(), dests)
        })
        .collect::<HashMap<_, _>>()
}

fn solve(input: &str) -> i64 {
    let graph = parse_graph(input);
    pathfinding::directed::count_paths::count_paths(
        "you",
        |n| graph.get(*n).unwrap().iter().map(|s| s.as_str()),
        |n| *n == "out",
    ) as i64
}

fn solve_part2(input: &str) -> i64 {
    let graph = parse_graph(input);

    let count = |src: &str, dst: &str| {
        let empty_bloody_hell = Vec::new();
        let ret = pathfinding::directed::count_paths::count_paths(
            src,
            |n| {
                graph
                    .get(*n)
                    .unwrap_or(&empty_bloody_hell)
                    .iter()
                    .map(|s| s.as_str())
            },
            |n| *n == dst,
        ) as i64;
        println!("{src} -> {dst} = {ret} paths");

        ret
    };

    // start -> req1 -> req2 -> dst
    let a = count("svr", "fft") * count("fft", "dac") * count("dac", "out");

    // start -> req2 -> req1 -> dst
    let b = count("svr", "dac") * count("dac", "fft") * count("fft", "out");

    dbg!(a) + dbg!(b)
}
