pub use itertools;
use owo_colors::OwoColorize;
use std::borrow::Cow;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PartNumber {
    Part1 = 1,
    Part2 = 2,
}

#[derive(Debug, Clone)]
pub enum SolutionResult {
    Int(i64),
    String(String),
}

pub trait Solution {
    fn solve(input: &str, part: PartNumber) -> impl Into<SolutionResult>;
}

pub enum SolutionInput {
    FullInput,
    Example(&'static str),
}

pub type CaseEntrypointFn = unsafe extern "C" fn(
    input_ptr: *const u8,
    input_len_bytes: usize,
    part_filter: u8,
    case_filter: u32,
    solutions_only: bool,
) -> bool;

#[macro_export]
macro_rules! solution {
    ($solution:ty, $cases:expr) => {
        #[no_mangle]
        pub extern "C" fn run_cases_entrypoint(
            input_ptr: *const u8,
            input_len_bytes: usize,
            part_filter: u8,
            case_filter: u32,
            solutions_only: bool,
        ) -> bool {
            let input = unsafe {
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                    input_ptr,
                    input_len_bytes,
                ))
            };

            let part = match part_filter {
                0 => None,
                1 => Some(common::PartNumber::Part1),
                2 => Some(common::PartNumber::Part2),
                _ => panic!("invalid part number {part_filter}"),
            };

            $crate::run_cases::<$solution>(input, &$cases, part, case_filter, solutions_only)
        }
    };
}

pub fn run_cases<S: Solution>(
    input: &str,
    cases: &[(PartNumber, SolutionInput, Option<SolutionResult>)],
    part_filter: Option<PartNumber>,
    case_filter: u32,
    solutions_only: bool,
) -> bool {
    let mut failed = 0;
    let mut total = 0;
    let mut all_passed = true;

    for (i, (part, case_input, expected)) in cases.iter().enumerate() {
        if part_filter.is_some() && Some(*part) != part_filter {
            continue;
        }

        if case_filter != 0 && (i + 1) as u32 != case_filter {
            continue;
        }

        if solutions_only && !matches!(case_input, SolutionInput::FullInput) {
            continue;
        }

        total += 1;

        let (input, wat) = match case_input {
            SolutionInput::FullInput => (input, "input  "),
            SolutionInput::Example(example) => (*example, "example"),
        };
        let result = S::solve(input, *part).into();

        match expected.clone() {
            Some(expected) if expected == result => {
                println!("\n{}", "═".repeat(80).bright_blue());
                println!(
                    "   {} {} case #{} for part {part:?} {wat}: {}",
                    "✓",
                    "PASS".green().bold(),
                    i + 1,
                    expected.bright_green().bold()
                );
                println!("{}\n", "═".repeat(80).bright_blue());
            }
            Some(expected) => {
                failed += 1;
                println!("\n{}", "═".repeat(80).bright_red());
                println!(
                    "   {} {} case #{} for part {part:?} {wat}: expected {}, got {}",
                    "✗",
                    "FAIL".red().bold(),
                    i + 1,
                    expected.bright_yellow().bold(),
                    result.bright_red().bold()
                );
                println!("{}\n", "═".repeat(80).bright_red());
                all_passed = false;
            }
            None => {
                println!(
                    "{} {} case #{} for part {part:?} {wat}: {}",
                    "?",
                    "INFO".bright_yellow(),
                    i + 1,
                    result.bright_white()
                );
            }
        }
    }

    if total > 0 {
        println!(
            "\n{} {} of {} tests passed",
            "Results:".bold(),
            (total - failed).green(),
            total,
        );
    }

    all_passed
}

pub fn lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
}

pub fn solution_part1(
    answer: Option<impl Into<SolutionResult>>,
) -> (PartNumber, SolutionInput, Option<SolutionResult>) {
    (
        PartNumber::Part1,
        SolutionInput::FullInput,
        answer.map(|a| a.into()),
    )
}

pub fn solution_part2(
    answer: Option<impl Into<SolutionResult>>,
) -> (PartNumber, SolutionInput, Option<SolutionResult>) {
    (
        PartNumber::Part2,
        SolutionInput::FullInput,
        answer.map(|a| a.into()),
    )
}

pub fn example_part1(
    answer: impl Into<SolutionResult>,
    input: &'static str,
) -> (PartNumber, SolutionInput, Option<SolutionResult>) {
    (
        PartNumber::Part1,
        SolutionInput::Example(input),
        Some(answer.into()),
    )
}

pub fn example_part2(
    answer: impl Into<SolutionResult>,
    input: &'static str,
) -> (PartNumber, SolutionInput, Option<SolutionResult>) {
    (
        PartNumber::Part2,
        SolutionInput::Example(input),
        Some(answer.into()),
    )
}

impl From<i64> for SolutionResult {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<String> for SolutionResult {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for SolutionResult {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl PartialEq<Self> for SolutionResult {
    fn eq(&self, other: &Self) -> bool {
        let a = match self {
            Self::Int(i) => Cow::Owned(i.to_string()),
            Self::String(s) => Cow::Borrowed(s),
        };

        let b = match other {
            Self::Int(i) => Cow::Owned(i.to_string()),
            Self::String(s) => Cow::Borrowed(s),
        };

        a == b
    }
}

impl Eq for SolutionResult {}

impl Display for SolutionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => i.fmt(f),
            Self::String(s) => s.fmt(f),
        }
    }
}

pub mod prelude {
    pub use crate::{
        example_part1, example_part2, lines, solution, solution_part1, solution_part2, PartNumber,
        Solution, SolutionInput, SolutionResult,
    };

    pub use itertools::{self, Itertools};
}
