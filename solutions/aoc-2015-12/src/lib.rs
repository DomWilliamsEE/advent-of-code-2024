use common::prelude::*;
use serde_json::Value;

pub struct Day12_2015;

impl Solution for Day12_2015 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => find_numbers(input, None),
            PartNumber::Part2 => find_numbers(input, Some("red")),
        }
    }
}

solution!(
    Day12_2015,
    [
        solution_part1(Some(191164)),
        example_part1(6, r#"{"a":2,"b":4}"#),
        solution_part2(Some(87842)),
        example_part2(4, r#"[1,{"c":"red","b":2},3]"#),
    ]
);

#[test]
fn test_build_2015_12() {}

// -----

fn find_numbers(input: &str, skip_key: Option<&str>) -> i64 {
    assert_eq!(lines(input).count(), 1);
    let json = serde_json::from_str::<Value>(input).unwrap();

    fn recurse(json: &Value, skip_key: Option<&str>) -> i64 {
        match json {
            Value::Object(obj)
                if skip_key.map(|s| obj.values().any(|v| v.as_str() == Some(s))) == Some(true) =>
            {
                0
            }
            Value::Object(obj) => obj.values().map(|v| recurse(v, skip_key)).sum(),
            Value::Array(arr) => arr.iter().map(|v| recurse(v, skip_key)).sum(),
            Value::Number(num) => num.as_i64().unwrap(),
            _ => 0,
        }
    }

    recurse(&json, skip_key)
}
