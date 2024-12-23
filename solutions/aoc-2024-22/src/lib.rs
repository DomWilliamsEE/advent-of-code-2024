use common::prelude::*;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::iter::{once, Iterator};
use std::sync::atomic::AtomicUsize;

pub struct Day22_2024;

impl Solution for Day22_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => lines(input)
                .map(|line| {
                    let res = iter_secret_numbers(line).nth(1999).unwrap();
                    println!("{line}: {res}");
                    res
                })
                .sum::<i64>(),
            PartNumber::Part2 => part2(input),
        }
    }
}

solution!(
    Day22_2024,
    [
        example_part1(37327623, "1\n10\n100\n2024"),
        solution_part1(Some(14691757043)),
        solution_part2(Some(1831)),
        example_part2(23, "1\n2\n3\n2024"),
    ]
);

#[test]
fn test_build_2024_22() {}

// -----

fn iter_secret_numbers(input: &str) -> impl Iterator<Item = i64> {
    let mut last: i64 = input.parse().unwrap();
    std::iter::from_fn(move || {
        let mut secret = last;

        macro_rules! mix_and_prune {
            ($x:expr) => {{
                secret = secret ^ $x;
                secret %= 16777216;
            }};
        }

        mix_and_prune!(secret * 64);
        mix_and_prune!(secret / 32);
        mix_and_prune!(secret * 2048);

        last = secret;
        Some(secret)
    })
}

fn iter_price_changes(buyer: &str) -> impl Iterator<Item = (i64, i8)> {
    once(buyer.parse().unwrap())
        .chain(iter_secret_numbers(buyer).take(2000))
        .map(|p| (p % 10) as i8)
        .tuple_windows()
        .map(|(a, b)| (b as i64, b - a))
}

fn part2(input: &str) -> i64 {
    let seq_occurrences = lines(input)
        .map(|buyer| {
            let price_changes = iter_price_changes(buyer);

            price_changes
                .map(|(_, change)| change)
                .tuple_windows()
                .map(|(a, b, c, d)| [a, b, c, d])
        })
        .fold(HashSet::new(), |mut a, b| {
            a.extend(b);
            a
        });

    let all_price_changes = lines(input)
        .map(|buyer| iter_price_changes(buyer).collect_vec())
        .collect_vec();

    let sz = seq_occurrences.len();
    let done = AtomicUsize::new(0);
    let res = seq_occurrences
        .into_par_iter()
        .fold(
            || HashMap::new(),
            |mut acc, seq| {
                all_price_changes.iter().for_each(|buyer| {
                    let total_bananas = buyer.windows(4).find_map(|window| {
                        if window[0].1 == seq[0]
                            && window[1].1 == seq[1]
                            && window[2].1 == seq[2]
                            && window[3].1 == seq[3]
                        {
                            Some(window[3].0)
                        } else {
                            None
                        }
                    });

                    if let Some(total_bananas) = total_bananas {
                        *acc.entry(seq).or_default() += total_bananas;
                    }
                });

                let n = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                println!(
                    "done with another {n}/{sz} sequences, {:.2}%",
                    (n as f64 / sz as f64) * 100.0
                );

                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut a, b| {
                a.extend(b);
                a
            },
        );
    res.values().max().copied().unwrap()
}

#[test]
fn test_secret() {
    println!("{:?}", iter_secret_numbers("123").take(10).collect_vec());
    assert_eq!(
        iter_secret_numbers("123").take(3).collect_vec(),
        vec![15887950, 16495136, 527345]
    );
}
