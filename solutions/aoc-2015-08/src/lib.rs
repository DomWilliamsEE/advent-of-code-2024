use common::prelude::*;

pub struct Day08_2015;

impl Solution for Day08_2015 {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult> {
        match part {
            PartNumber::Part1 => lines(input)
                .map(|s| len(s) - len(&parse_string_literal(s)))
                .sum::<usize>() as i64,
            PartNumber::Part2 => lines(input)
                .map(|s| len(&escape_string_literal(s)) - len(s))
                .sum::<usize>() as i64,
        }
    }
}

solution!(
    Day08_2015,
    [
        solution_part1(Some(1333)),
        example_part1(
            12,
            r#"""
        "abc"
        "aaa\"aaa"
        "\x27""#
        ),
        solution_part2(Some(2046)),
        example_part2(
            19,
            r#"""
        "abc"
        "aaa\"aaa"
        "\x27""#
        ),
    ]
);

#[test]
fn test_build_2015_08() {}

// -----

fn len(s: &str) -> usize {
    s.chars().count()
}

fn parse_string_literal(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();

    assert_eq!(chars.next(), Some('"'));

    loop {
        let c = match chars.next() {
            None => panic!("ended without closing quote"),
            Some('"') => break,
            Some(c) => c,
        };

        if c == '\\' {
            match chars.next() {
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some('x') => {
                    let a = chars.next().unwrap();
                    let b = chars.next().unwrap();
                    let hex =
                        u32::from_str_radix(format!("{}{}", a, b).as_str(), 16).expect("bad hex");

                    let c = char::from_u32(hex).expect("bad hex");
                    out.push(c);
                }
                _ => panic!("unexpected escape char"),
            }
        } else {
            out.push(c);
        }
    }

    println!("{s} ({}) -> {out} ({})", len(s), len(&out));

    out
}

fn escape_string_literal(s: &str) -> String {
    let mut out = String::new();
    out.push('"');

    let mut chars = s.chars().peekable();
    loop {
        let Some(c) = chars.next() else {
            break;
        };

        match c {
            '"' => out.push_str("\\\""),
            '\\' => {
                out.push_str("\\\\");

                let escaped = *chars.peek().unwrap();
                if escaped == 'x' {
                    for _ in 0..3 {
                        let c = chars.next().unwrap();
                        out.push(c);
                    }
                }
            }
            _ => out.push(c),
        }
    }

    out.push('"');

    println!("{s} ({}) -> {out} ({})", len(s), len(&out));
    out
}

#[test]
fn test_examples() {
    assert_eq!(parse_string_literal(r#""abc""#), "abc");
    assert_eq!(parse_string_literal(r#""aaa\"aaa""#), "aaa\"aaa");
    assert_eq!(parse_string_literal(r#""\x27""#), "\x27");
}
