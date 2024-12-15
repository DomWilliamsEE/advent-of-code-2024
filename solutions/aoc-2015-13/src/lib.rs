use common::prelude::*;
use std::collections::HashMap;

pub struct Day13_2015;

impl Solution for Day13_2015 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => find_best_order(&parse_preferences(input)),
            PartNumber::Part2 => find_best_order(&parse_preferences(input).with_self_added()),
        }
    }
}

const PART1_EXAMPLE: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

solution!(
    Day13_2015,
    [
        solution_part1(Some(733)),
        example_part1(330, PART1_EXAMPLE),
        solution_part2(Some(725)),
    ]
);

#[test]
fn test_build_2015_13() {}

// -----

#[derive(Debug)]
struct Preferences {
    map: HashMap<String, HashMap<String, i8>>,
}

fn parse_preferences(input: &str) -> Preferences {
    let mut preferences: HashMap<String, HashMap<String, i8>> = HashMap::new();
    lines(input).for_each(|line| {
        let mut words = line.split_whitespace();
        let name = words.next().unwrap();
        let gain_or_lose = words.nth(1).unwrap();
        let amount = words.next().unwrap();
        let neighbour = words.last().unwrap().trim_end_matches('.');

        preferences.entry(name.to_owned()).or_default().insert(
            neighbour.to_owned(),
            amount.parse::<i8>().unwrap() * if gain_or_lose == "gain" { 1 } else { -1 },
        );
    });

    Preferences { map: preferences }
}

impl Preferences {
    fn score_permutation(&self, order: &[&&str]) -> i64 {
        let mut total = 0;

        let pairs = order
            .iter()
            .copied()
            .cycle()
            .tuple_windows::<(_, _, _)>()
            .take(order.len());

        for (left, person, right) in pairs {
            let prefs = &self.map[*person];
            let left_prefs = prefs.get(*left).copied().unwrap_or(0);
            let right_prefs = prefs.get(*right).copied().unwrap_or(0);

            // println!("for {person}: left_prefs={left_prefs}, right_prefs={right_prefs}");
            total += left_prefs as i64 + right_prefs as i64;
        }

        total
    }

    fn with_self_added(mut self) -> Self {
        self.map.insert("self".to_owned(), HashMap::new());
        self
    }
}

fn find_best_order(preferences: &Preferences) -> i64 {
    let names = preferences
        .map
        .keys()
        .map(|name| name.as_str())
        .collect::<Vec<_>>();

    let permutations = names.iter().permutations(names.len());
    let max_score = permutations
        .map(|p| (p.clone(), preferences.score_permutation(&p)))
        .max_by_key(|(_, score)| *score)
        .unwrap();

    println!("{max_score:?}");
    max_score.1
}
