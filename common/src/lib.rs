pub use itertools;
use owo_colors::OwoColorize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum PartNumber {
    Part1 = 1,
    Part2 = 2,
}

pub trait Solution {
    fn solve(input: &str, part: PartNumber) -> i64;
}

pub enum SolutionInput {
    FullInput,
    Example(&'static str),
}

pub type SolutionEntrypointFn =
    unsafe extern "C" fn(input_ptr: *const u8, input_len_bytes: usize, part: u8) -> i64;

pub type ExemplarEntrypointFn = unsafe extern "C" fn(
    input_ptr: *const u8,
    input_len_bytes: usize,
    part_filter: u8,
    exemplar_filter: u32,
) -> bool;

#[macro_export]
macro_rules! solution {
    ($solution:ty, $exemplars:expr) => {
        #[no_mangle]
        pub extern "C" fn solution_entrypoint(
            input_ptr: *const u8,
            input_len_bytes: usize,
            part: u8,
        ) -> i64 {
            let part = match part {
                1 => common::PartNumber::Part1,
                2 => common::PartNumber::Part2,
                _ => panic!("invalid part number {part}"),
            };
            let input = unsafe {
                std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                    input_ptr,
                    input_len_bytes,
                ))
            };

            <$solution>::solve(input, part)
        }

        #[no_mangle]
        pub extern "C" fn run_exemplars_entrypoint(
            input_ptr: *const u8,
            input_len_bytes: usize,
            part_filter: u8,
            exemplar_filter: u32,
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

            $crate::run_exemplars::<$solution>(input, &$exemplars, part, exemplar_filter)
        }
    };
}

pub fn run_exemplars<S: Solution>(
    input: &str,
    exemplars: &[(PartNumber, SolutionInput, Option<i64>)],
    part_filter: Option<PartNumber>,
    exemplar_filter: u32,
) -> bool {
    let mut failed = 0;
    let mut total = 0;
    let mut all_passed = true;

    for (i, (part, exemplar_input, expected)) in exemplars.iter().enumerate() {
        if part_filter.is_some() && Some(*part) != part_filter {
            continue;
        }

        if exemplar_filter != 0 && (i + 1) as u32 != exemplar_filter {
            continue;
        }

        total += 1;

        let (input, wat) = match exemplar_input {
            SolutionInput::FullInput => (input, "input  "),
            SolutionInput::Example(example) => (*example, "example"),
        };
        let result = S::solve(input, *part);

        match *expected {
            Some(expected) if expected == result => {
                println!("\n{}", "═".repeat(80).bright_blue());
                println!(
                    "   {} {} exemplar #{} for part {part:?} {wat}: {}",
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
                    "   {} {} exemplar #{} for part {part:?} {wat}: expected {}, got {}",
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
                    "{} {} exemplar #{} for part {part:?} {wat}: {}",
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

pub fn example_part1(answer: i64, input: &'static str) -> (PartNumber, SolutionInput, Option<i64>) {
    (
        PartNumber::Part1,
        SolutionInput::Example(input),
        Some(answer),
    )
}

pub fn example_part2(answer: i64, input: &'static str) -> (PartNumber, SolutionInput, Option<i64>) {
    (
        PartNumber::Part2,
        SolutionInput::Example(input),
        Some(answer),
    )
}
