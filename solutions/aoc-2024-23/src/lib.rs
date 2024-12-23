use common::prelude::*;
use std::collections::{HashMap, HashSet};
use std::iter::{Extend, IntoIterator, Iterator};
use std::str::FromStr;

pub struct Day23_2024;

impl Solution for Day23_2024 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => input
                .parse::<Connections>()
                .unwrap()
                .find_three_connections()
                .filter(|computers| computers.iter().any(|c| c.starts_with('t')))
                .count()
                .to_string(),
            PartNumber::Part2 => input
                .parse::<Connections>()
                .unwrap()
                .find_longest_chain()
                .join(","),
        }
    }
}

solution!(
    Day23_2024,
    [
        example_part1(7, PART1_EXAMPLE),
        solution_part1(Some(1485)),
        example_part2("co,de,ka,ta", PART1_EXAMPLE),
        solution_part2(None::<i64>),
    ]
);

const PART1_EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

#[test]
fn test_build_2024_23() {}

// -----

struct Connections {
    adjacency: HashMap<String, HashSet<String>>,
}

impl FromStr for Connections {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let connections = lines(s)
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                if a < b {
                    (a.to_string(), b.to_string())
                } else {
                    (b.to_string(), a.to_string())
                }
            })
            .collect::<HashSet<_>>();

        let adj = connections
            .iter()
            .fold(HashMap::<_, HashSet<String>>::new(), |mut acc, n| {
                acc.entry(n.0.clone()).or_default().insert(n.1.clone());
                acc.entry(n.1.clone()).or_default().insert(n.0.clone());
                acc
            });

        Ok(Self { adjacency: adj })
    }
}

impl Connections {
    fn find_three_connections(&self) -> impl Iterator<Item = [String; 3]> + '_ {
        let mut sets = HashSet::new();
        for (a, an) in self.adjacency.iter() {
            for b in an {
                for c in self.adjacency[b].iter() {
                    if an.contains(c) {
                        let mut three = [a.clone(), b.clone(), c.clone()];
                        three.sort();
                        sets.insert(three);
                    }
                }
            }
        }

        sets.into_iter()
    }

    fn find_longest_chain(&self) -> impl Iterator<Item = String> + '_ {
        fn bron_kerbosch(
            r: &mut HashSet<String>,
            p: &mut HashSet<String>,
            x: &mut HashSet<String>,
            adj: &HashMap<String, HashSet<String>>,
            res: &mut HashSet<String>,
        ) {
            if p.is_empty() && x.is_empty() {
                if r.len() > res.len() {
                    res.clear();
                    res.extend(r.iter().cloned());
                }
                return;
            }

            let pivot = p
                .union(x)
                .max_by_key(|v| p.intersection(&adj[*v]).count())
                .unwrap();

            let candidates = p.difference(&adj[pivot]).cloned().collect_vec();
            for v in candidates {
                let neighbors = &adj[&v];
                r.insert(v.clone());
                p.remove(&v);

                let mut new_p: HashSet<_> = p.intersection(neighbors).cloned().collect();
                let mut new_x: HashSet<_> = x.intersection(neighbors).cloned().collect();

                bron_kerbosch(r, &mut new_p, &mut new_x, adj, res);

                r.remove(&v);
                x.insert(v);
            }
        }

        let mut res = HashSet::new();
        let mut r = HashSet::new();
        let mut p = self.adjacency.keys().cloned().collect();
        let mut x = HashSet::new();

        bron_kerbosch(&mut r, &mut p, &mut x, &self.adjacency, &mut res);
        res.into_iter().sorted()
    }
}
