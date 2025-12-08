use common::prelude::*;
use glam::{u64vec3, U64Vec3};
use std::cmp::Reverse;
use std::collections::HashMap;

pub struct Day08_2025;

impl Solution for Day08_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve(input) as i64,
            PartNumber::Part2 => solve_part2(input) as i64,
        }
    }
}

solution!(
    Day08_2025,
    [
        example_part1(
            40,
            "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        ),
        solution_part1(Some(175440)),
        example_part2(
            25272,
            "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        ),
        solution_part2(Some(3200955921)),
    ]
);

#[test]
fn test_build_2025_08() {}

// -----

fn circuit_loop(input: &str, limited: bool) -> (Vec<usize>, [U64Vec3; 2]) {
    let positions = lines(input)
        .map(|s| {
            let coords = s
                .split(',')
                .map(|num| num.parse::<u64>().unwrap())
                .collect_vec();
            u64vec3(coords[0], coords[1], coords[2])
        })
        .collect_vec();

    let mut circuits = (0..positions.len()).collect_vec();

    let mut circuit_counts = vec![1usize; positions.len()];
    let mut histogram = HashMap::new();

    let all_pairs = (0..positions.len())
        .into_iter()
        .combinations(2)
        .sorted_unstable_by_key(|x| (positions[x[0]] - positions[x[1]]).length_squared())
        .collect_vec();

    let limit = if limited && positions.len() == 20 {
        10 // example
    } else if limited {
        1000 // part 1
    } else {
        all_pairs.len() // part 2
    };

    let mut last_pair = None;
    for pair_idx in 0..limit {
        let pair = &all_pairs[pair_idx];
        let a_idx = pair[0];
        let b_idx = pair[1];

        let a_pos = positions[a_idx];
        let b_pos = positions[b_idx];

        assert_ne!(a_pos, b_pos);

        let a_circuit = circuits[a_idx];
        let b_circuit = circuits[b_idx];

        println!(
            "consider {:3},{:3},{:3} -> {:3},{:3},{:3}",
            a_pos.x, a_pos.y, a_pos.z, b_pos.x, b_pos.y, b_pos.z
        );

        if a_circuit == b_circuit {
            println!("same circuit already");
            continue;
        }

        // let a_circuit_count = circuit_counts[a_circuit];
        // let b_circuit_count = circuit_counts[b_circuit];

        let (src, dst) = (a_circuit, b_circuit);
        for c in circuits.iter_mut() {
            if *c == dst {
                *c = src;
            }
        }

        circuit_counts.iter_mut().for_each(|c| *c = 0);
        for c in circuits.iter() {
            circuit_counts[*c] += 1;
        }

        histogram.clear();
        for c in circuit_counts.iter() {
            *histogram.entry(*c).or_insert(0usize) += 1;
        }

        let histogram_str = histogram
            .iter()
            .sorted_unstable_by_key(|x| Reverse(x.1))
            .filter(|(_, v)| **v > 0)
            .map(|(k, v)| format!("{v}x {k}"))
            .collect_vec()
            .join(", ");

        println!("CONNECTED {histogram_str}");

        last_pair = Some([a_pos, b_pos]);

        if circuits.iter().all_equal() {
            println!("ALL CONNECTED!");
            break;
        }
    }

    (circuit_counts, last_pair.unwrap())
}

fn solve(input: &str) -> usize {
    let (circuit_counts, _) = circuit_loop(input, true);

    let sizes_sorted = circuit_counts
        .into_iter()
        .filter(|&c| c > 0)
        .sorted_unstable_by_key(|&c| Reverse(c))
        .take(3)
        .collect_vec();

    println!("final circuit sizes: {:?}", sizes_sorted);

    sizes_sorted.iter().product()
}

fn solve_part2(input: &str) -> u64 {
    let (_, last_pair) = circuit_loop(input, false);

    println!("last connected pair: {:?}", last_pair);

    last_pair[0].x * last_pair[1].x
}
