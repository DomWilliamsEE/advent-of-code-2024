use common::prelude::*;
use good_lp::{
    default_solver, variable, Expression, ProblemVariables, Solution as Gtfo, SolverModel,
};
use smallvec::{smallvec, SmallVec};

pub struct Day10_2025;

impl Solution for Day10_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => solve(input),
            PartNumber::Part2 => solve_part2(input),
        }
    }
}

solution!(
    Day10_2025,
    [
        example_part1(
            7,
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
        ),
        solution_part1(Some(494)),
        example_part2(
            33,
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
        ),
        solution_part2(Some(19235))
    ]
);

#[test]
fn test_build_2025_10() {}

// -----

/// Gross
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
struct Lights(Vec<bool>);

#[derive(Debug, Clone)]
struct Machine {
    goal_lights: Lights,
    buttons: Vec<Button>,
    goal_counters: Vec<u32>,
}

#[derive(Debug, Clone)]
struct Button {
    // light indices or joltage counters
    toggles: Vec<u8>,
}

fn parse(input: &str) -> Vec<Machine> {
    lines(input)
        .map(|s| {
            let mut buttons = vec![];
            let mut lights = vec![];
            let mut joltage_counters = vec![];
            for chunk in s.split_whitespace() {
                match chunk.chars().next().unwrap() {
                    '[' => {
                        for c in chunk.chars().skip(1) {
                            let on = match c {
                                '.' => false,
                                '#' => true,
                                ']' => break,
                                _ => unreachable!(),
                            };

                            lights.push(on);
                        }
                    }

                    '(' => {
                        let mut button_lights = Vec::new();
                        for num in chunk.split(',') {
                            let num = num
                                .trim_matches(|c| c == '(' || c == ')')
                                .parse::<u8>()
                                .unwrap();
                            button_lights.push(num);
                        }

                        buttons.push(Button {
                            toggles: button_lights,
                        });
                    }

                    '{' => {
                        assert!(joltage_counters.is_empty());
                        for num in chunk.split(',') {
                            let num = num
                                .trim_matches(|c| c == '{' || c == '}')
                                .parse::<u32>()
                                .unwrap();
                            joltage_counters.push(num);
                        }
                    }
                    _ => unreachable!(),
                }
            }

            assert!(!buttons.is_empty());
            assert!(!lights.is_empty());
            assert!(!joltage_counters.is_empty());
            Machine {
                goal_lights: Lights(lights),
                buttons,
                goal_counters: joltage_counters,
            }
        })
        .collect_vec()
}

impl Machine {
    fn solve(&self) -> usize {
        println!("solving machine: {:?}", self);
        let buttons = self.buttons.clone();
        let start = Lights(vec![false; self.goal_lights.0.len()]);
        pathfinding::directed::dijkstra::dijkstra(
            &start,
            |state| {
                let state = state.clone();
                buttons.iter().map(move |button| {
                    let mut next = state.clone();
                    for &light_idx in &button.toggles {
                        next.0[light_idx as usize] = !next.0[light_idx as usize];
                    }
                    (next, 1)
                })
            },
            |state| *state == self.goal_lights,
        )
        .expect("no solution found")
        .1
    }

    fn solve_part2(&self) -> usize {
        println!("solving machine: {:?}", self);

        // damn it maths you win again
        let mut problem = ProblemVariables::new();
        let button_vars = self
            .buttons
            .iter()
            .map(|_| problem.add(variable().integer().min(0)))
            .collect_vec();

        let total_presses: Expression = button_vars.iter().cloned().sum();
        let mut model = problem.minimise(total_presses).using(default_solver);

        for (i, counter_goal) in self.goal_counters.iter().enumerate() {
            let mut expr = Expression::from(0);
            for (button, var) in self.buttons.iter().zip(button_vars.iter()) {
                if button.toggles.contains(&(i as u8)) {
                    expr = expr + var;
                }
            }
            model.add_constraint(expr.eq(*counter_goal));
        }

        let solution = model.solve().unwrap();
        let presses = button_vars
            .iter()
            .map(|var| solution.value(*var).round() as usize)
            .collect_vec();
        let sum = presses.iter().sum();
        println!("maths said {presses:?} == {sum}");

        // verify
        let mut actual_counters = vec![0u32; self.goal_counters.len()];
        for (button, &count) in self.buttons.iter().zip(presses.iter()) {
            for &idx in &button.toggles {
                actual_counters[idx as usize] += count as u32;
            }
        }
        assert_eq!(actual_counters, self.goal_counters, "wrong!");

        return sum;

        let buttons = self.buttons.clone();
        let start: SmallVec<u32, 16> = smallvec![0u32; self.goal_counters.len()];
        let goal = self
            .goal_counters
            .clone()
            .into_iter()
            .collect::<SmallVec<_, 16>>();

        let mut n = 0usize;
        pathfinding::directed::astar::astar(
            &start,
            |state| {
                // for (cur, goal) in state.iter().zip(self.goal_counters.iter()) {
                //     if cur > goal {
                //         return smallvec![]; // impossilbe
                //     }
                // }
                //
                let state = state.clone();
                n += 1;
                if n % 5000_000 == 0 {
                    println!("explored {n} states, state is {state:?}, goal is {goal:?}",);
                }
                let vec = buttons
                    .iter()
                    .filter_map(move |button| {
                        let mut next = state.clone();
                        for &idx in &button.toggles {
                            next[idx as usize] += 1;

                            if next[idx as usize] > self.goal_counters[idx as usize] {
                                return None; // impossible
                            }
                        }
                        Some((next, 1))
                    })
                    .collect::<SmallVec<_, 16>>();

                // println!("len of next states: {}", vec.len());
                vec
            },
            |state| {
                state
                    .iter()
                    .zip(self.goal_counters.iter())
                    .map(|(cur, goal)| goal - cur)
                    .sum::<u32>() as usize
            },
            |state| *state == goal,
        )
        .expect("no solution found")
        .1
    }
}

fn solve(input: &str) -> i64 {
    let machines = parse(input);
    machines.iter().map(|m| m.solve()).sum::<usize>() as i64
}

fn solve_part2(input: &str) -> i64 {
    let machines = parse(input);
    println!("parsed {} machines", machines.len());
    machines.iter().map(|m| m.solve_part2()).sum::<usize>() as i64
}
