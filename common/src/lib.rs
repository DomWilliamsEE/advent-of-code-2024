pub use itertools;

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

pub type ExemplarEntrypointFn =
    unsafe extern "C" fn(input_ptr: *const u8, input_len_bytes: usize, part_filter: u8) -> bool;

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

            $crate::run_exemplars::<$solution>(input, &$exemplars, part)
        }
    };
}

pub fn run_exemplars<S: Solution>(
    input: &str,
    exemplars: &[(PartNumber, SolutionInput, Option<i64>)],
    part_filter: Option<PartNumber>,
) -> bool {
    let mut all_passed = true;
    for (i, (part, exemplar_input, expected)) in exemplars.iter().enumerate() {
        if part_filter.is_some() && Some(*part) != part_filter {
            continue;
        }

        // println!("running exemplar #{} for part {part:?}", i + 1);

        let (input, wat) = match exemplar_input {
            SolutionInput::FullInput => (input, "input  "),
            SolutionInput::Example(example) => (*example, "example"),
        };
        let result = S::solve(input, *part);

        match *expected {
            Some(expected) if expected == result => {
                println!(
                    "exemplar #{} for part {part:?} {wat} PASSED: {expected}",
                    i + 1
                );
            }
            Some(expected) => {
                println!(
                    "exemplar #{} for part {part:?} {wat} FAILED: expected {expected}, got {result}",
                    i + 1
                );
                all_passed = false;
            }
            None => {
                println!(
                    "exemplar #{} for part {part:?} {wat} returned {result}",
                    i + 1
                );
            }
        }
    }

    all_passed
}
