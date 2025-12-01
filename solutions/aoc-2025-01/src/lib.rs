use common::prelude::*;

pub struct Day01_2025;

impl Solution for Day01_2025 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => turn(50, input).1 as i64,
            PartNumber::Part2 => {
                let (passed, landed) = turn(50, input);
                (passed + landed) as i64
            }
        }
    }
}

solution!(
    Day01_2025,
    [
        example_part1(
            3,
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        ),
        solution_part1(Some(1132)),
        example_part2(
            6,
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        ),
        solution_part2(Some(6623)),
    ]
);

#[test]
fn test_build_2025_01() {}

// -----

fn turn(start: u8, input: &str) -> (u32, u32) {
    let insns = lines(input).map(|s| {
        let (dir, n) = s.split_at(1);
        let n: i32 = n.parse().unwrap();
        match dir {
            "L" => n * -1,
            "R" => n,
            _ => unreachable!(),
        }
    });

    let mut current = start;
    let mut passed_by_count = 0;
    let mut landed_on_count = 0;

    for insn in insns {
        let step = if insn.is_negative() { -1 } else { 1 };
        let passed_by = (0..insn.abs())
            .map(|i| (current as i32 + (step * i)).rem_euclid(100) as u8)
            .filter(|x| *x == 0);

        passed_by_count += passed_by.count() as u32;

        current = (current as i32 + insn).rem_euclid(100) as u8;
        if current == 0 {
            landed_on_count += 1;
            passed_by_count -= 1; // already counted as being landed on
        }
    }

    (passed_by_count, landed_on_count)
}
