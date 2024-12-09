use crate::day09::{part_one, part_two, PART1_EXAMPLE, PART2_EXAMPLE};
use std::borrow::Cow;

fn main() {
    let part = 2;
    let mut example = true;
    example = false;

    let input = if example {
        Cow::Borrowed(if part == 1 {
            PART1_EXAMPLE
        } else if !PART2_EXAMPLE.is_empty() {
            PART2_EXAMPLE
        } else {
            PART1_EXAMPLE
        })
    } else {
        Cow::Owned(std::fs::read_to_string("src/day09-input").unwrap())
    };

    let result = if part == 1 {
        part_one(&input)
    } else {
        part_two(&input)
    };

    println!("{}", result);
}

// macro_rules! declare_day {
//     ($day:ident
//     ) => {
//         #[cfg(feature = std::stringify!($day))]
//         mod $day;;
//
//         #[cfg(feature = std::stringify!($day))]
//         use $day as today;
//     };
// }

#[cfg(feature = "day08")]
mod day08;
#[cfg(feature = "day09")]
mod day09;
#[cfg(feature = "day10")]
mod day10;
