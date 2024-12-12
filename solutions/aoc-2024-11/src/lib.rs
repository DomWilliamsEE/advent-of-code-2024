#![allow(dead_code)]

use crate::compress_ints_zstd::CompressedIntsZstd;
use common::itertools::Itertools;
use common::{example_part1, solution, PartNumber, Solution, SolutionInput};
use hashbrown::hash_map::Entry;
use hashbrown::HashMap;
use size_of::SizeOf;
use std::fmt::Debug;

pub struct Day11_2024;

impl Solution for Day11_2024 {
    fn solve(input: &str, part: PartNumber) -> i64 {
        match part {
            PartNumber::Part1 => count_recursively(input, 25),
            PartNumber::Part2 => count_recursively(input, 75),
        }
    }
}
solution!(
    Day11_2024,
    [
        (PartNumber::Part1, SolutionInput::FullInput, Some(188902)),
        example_part1(55312, "125 17"),
        (
            PartNumber::Part2,
            SolutionInput::FullInput,
            Some(223894720281135)
        ),
    ]
);

#[test]
fn test_build_2024_11() {}

// -----

struct Stones(CompressedIntsZstd);

fn count_recursively(input: &str, max_depth: usize) -> i64 {
    input
        .split_whitespace()
        .map(|s| iter_recursive(s.parse().unwrap(), max_depth))
        .sum::<usize>() as i64
}

fn iter_recursive(stone: u64, max_depth: usize) -> usize {
    fn recurse(
        stone: u64,
        this_depth: usize,
        max_depth: usize,
        cache: &mut HashMap<(u64, usize), usize>,
    ) -> usize {
        if let Some(count) = cache.get(&(stone, this_depth)) {
            return *count;
        }

        if this_depth == max_depth {
            return 1;
        }

        let ret = if stone == 0 {
            recurse(1, this_depth + 1, max_depth, cache)
        } else if (stone.ilog10() + 1) % 2 == 0 {
            let div = 10u64.pow((stone.ilog10() + 1) / 2);
            let a = stone % div;
            let b = stone / div;

            recurse(a, this_depth + 1, max_depth, cache)
                + recurse(b, this_depth + 1, max_depth, cache)
        } else {
            recurse(stone.wrapping_mul(2024), this_depth + 1, max_depth, cache)
        };

        cache.insert((stone, this_depth), ret);
        // println!("cache: {cache:?}");
        ret
    }

    let mut cache = HashMap::new();
    recurse(stone, 0, max_depth, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Stones;
    use common::itertools::Either;

    #[track_caller]
    fn check(input: &str, blinks: usize, expected: Either<usize, Vec<u64>>) {
        let actual_len = count_recursively(input, blinks);

        if let Either::Right(expected) = expected.clone() {
            let mut stones = Stones::new(input);
            stones.blink_n_times(blinks);
            assert_eq!(
                stones.0.iter().collect_vec(),
                expected,
                "recursive len {actual_len}"
            );
        }

        let expected_len = match expected {
            Either::Left(len) => len,
            Either::Right(expected) => expected.len(),
        };

        assert_eq!(actual_len, expected_len as i64);
    }

    #[test]
    fn test_blinking() {
        /*
        After 1 blink:
        253000 1 7

        After 2 blinks:
        253 0 2024 14168

        After 3 blinks:
        512072 1 20 24 28676032

        After 4 blinks:
        512 72 2024 2 0 2 4 2867 6032

        After 5 blinks:
        1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32

        After 6 blinks:
        2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
         */

        let input = "125 17";
        check(input, 1, Either::Right(vec![253000, 1, 7]));
        check(input, 2, Either::Right(vec![253, 0, 2024, 14168]));
        check(input, 3, Either::Right(vec![512072, 1, 20, 24, 28676032]));
        check(
            input,
            4,
            Either::Right(vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]),
        );
        check(
            input,
            5,
            Either::Right(vec![
                1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32,
            ]),
        );
        check(
            input,
            6,
            Either::Right(vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2,
            ]),
        );

        check(input, 25, Either::Left(55312));
    }
}

mod compressed_ints_small_separately {
    use bitvec::prelude::BitVec;

    use size_of::{Context, SizeOf};

    use std::ops::{Deref, DerefMut};

    #[derive(Default, SizeOf)]
    struct CompressedInts {
        small: Vec<u16>,
        big: Vec<u64>,
        indices_are_big: BitVecWrapped,
    }

    #[derive(Default)]
    struct BitVecWrapped(BitVec);

    impl Deref for BitVecWrapped {
        type Target = BitVec;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for BitVecWrapped {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl SizeOf for BitVecWrapped {
        fn size_of_children(&self, context: &mut Context) {
            context.add_vectorlike(self.0.len() / 8, self.0.capacity() / 8, 1);
        }
    }

    impl CompressedInts {
        fn push(&mut self, i: u64) {
            let big = if i < u16::MAX as u64 {
                self.small.push(i as u16);
                false
            } else {
                self.big.push(i);
                true
            };

            self.indices_are_big.push(big);
        }

        fn len(&self) -> usize {
            self.indices_are_big.len()
        }

        fn iter(&self) -> impl Iterator<Item = u64> + ExactSizeIterator + '_ {
            let mut small = self.small.iter();
            let mut big = self.big.iter();
            self.indices_are_big.iter().map(move |is_big| {
                if *is_big {
                    big.next().copied().unwrap()
                } else {
                    small.next().map(|i| *i as u64).unwrap()
                }
            })
        }

        fn print_stats(&self) {
            println!(
                "small: {}, big: {}, total len: {}",
                self.small.len(),
                self.big.len(),
                self.len()
            );

            let uncompressed_size = bytesize::ByteSize((self.len() * size_of::<u64>()) as u64);
            let compressed_size = bytesize::ByteSize(self.size_of().total_bytes() as u64);

            println!("uncompressed size: {uncompressed_size}, compressed size: {compressed_size}");
        }
    }

    impl FromIterator<u64> for CompressedInts {
        fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
            let mut ci = CompressedInts::default();
            for i in iter {
                ci.push(i);
            }
            ci
        }
    }

    #[cfg(test)]
    impl PartialEq<Vec<u64>> for CompressedInts {
        fn eq(&self, other: &Vec<u64>) -> bool {
            self.iter().collect_vec() == *other
        }
    }

    #[cfg(test)]
    impl Debug for CompressedInts {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.iter()).finish()
        }
    }

    #[test]
    fn test_compressed_ints() {
        let actual = vec![2, 5, 1, 500, 10, 800];
        let compressed = actual.iter().copied().collect::<CompressedInts>();

        assert_eq!(compressed.iter().collect_vec(), actual);
    }
}

mod compress_ints_zstd {
    use super::*;
    use std::io::{Cursor, Read, Write};
    use zstd::Encoder;

    enum Compressed {
        InProgress(Encoder<'static, Vec<u8>>),
        Done(Vec<u8>),
    }

    // #[derive(SizeOf)]
    pub struct CompressedIntsZstd {
        len: usize,
        enc: Compressed,
        buf: Vec<u64>,
    }

    impl FromIterator<u64> for CompressedIntsZstd {
        fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
            let mut ci = CompressedIntsZstd::default();
            for i in iter.into_iter() {
                ci.push(i);
            }
            ci.ensure_ready();
            ci
        }
    }

    impl Default for CompressedIntsZstd {
        fn default() -> Self {
            Self {
                len: 0,
                enc: Compressed::InProgress(Encoder::new(Vec::new(), 3).unwrap()),
                buf: Vec::new(),
            }
        }
    }

    impl CompressedIntsZstd {
        pub fn push(&mut self, i: u64) {
            self.buf.push(i);

            if self.buf.len() > 4096 {
                self.flush();
            }
        }

        fn flush(&mut self) {
            if self.buf.is_empty() {
                return;
            }
            match &mut self.enc {
                Compressed::InProgress(enc) => {
                    let bytes = unsafe { self.buf.align_to::<u8>().1 };
                    enc.write_all(bytes).unwrap();
                    self.len += self.buf.len();
                    self.buf.clear();
                }
                _ => panic!("already done"),
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn ensure_ready(&mut self) {
            self.flush();
            self.enc = match std::mem::replace(&mut self.enc, Compressed::Done(Vec::new())) {
                Compressed::InProgress(enc) => {
                    let vec = enc.finish().expect("finish failed");
                    Compressed::Done(vec)
                }
                Compressed::Done(vec) => Compressed::Done(vec),
            };
        }

        pub fn iter(&self) -> impl Iterator<Item = u64> + '_ {
            match &self.enc {
                Compressed::Done(vec) => {
                    let mut decoder = zstd::Decoder::new(Cursor::new(vec)).expect("decoder failed");

                    std::iter::from_fn(move || {
                        let mut buf = [0u8; 8];
                        match decoder.read_exact(&mut buf) {
                            Ok(()) => Some(u64::from_ne_bytes(buf)),
                            Err(_) => None,
                        }
                    })
                }
                _ => unreachable!(),
            }
        }

        pub fn print_stats(&self) {
            println!(
                "uncompressed size: {}, actual size: {}",
                bytesize::ByteSize((self.len * size_of::<u64>()) as u64),
                bytesize::ByteSize(match &self.enc {
                    Compressed::Done(vec) => vec.len() as u64,
                    Compressed::InProgress(_) => unreachable!(),
                })
            );
        }
    }

    impl PartialEq<Vec<u64>> for CompressedIntsZstd {
        fn eq(&self, other: &Vec<u64>) -> bool {
            self.iter().collect_vec() == *other
        }
    }
}

mod compress_ints_dupes {
    use super::*;

    #[derive(Default, SizeOf, Debug)]
    pub struct CompressedIntsDupes {
        ints: Vec<u64>,
        indices: Vec<u16>,
        lookup: HashMap<u64, u16>,
    }

    impl FromIterator<u64> for CompressedIntsDupes {
        fn from_iter<T: IntoIterator<Item = u64>>(iter: T) -> Self {
            let mut ci = CompressedIntsDupes::default();
            for i in iter.into_iter() {
                ci.push(i);
            }
            ci
        }
    }

    impl CompressedIntsDupes {
        pub fn push(&mut self, i: u64) {
            match self.lookup.entry(i) {
                Entry::Occupied(entry) => {
                    self.indices
                        .push((*entry.get()).try_into().expect("too big"));
                }
                Entry::Vacant(entry) => {
                    let index = self.ints.len();
                    self.ints.push(i);
                    let small_idx = u16::try_from(index).expect("too big");
                    self.indices.push(small_idx);
                    entry.insert(small_idx);
                }
            }
        }

        pub fn len(&self) -> usize {
            self.indices.len()
        }

        pub fn iter(&self) -> impl Iterator<Item = u64> + ExactSizeIterator + '_ {
            self.indices.iter().map(move |&i| self.ints[i as usize])
        }
        pub fn print_stats(&self) {
            let uncompressed_size = bytesize::ByteSize((self.len() * size_of::<u64>()) as u64);
            let compressed_size = bytesize::ByteSize(self.size_of().total_bytes() as u64);

            let uniques = self.ints.len();
            let total = self.indices.len();
            println!(
                "uniques: {}, total: {}, ratio: {}",
                uniques,
                total,
                uniques as f64 / total as f64
            );
            println!("uncompressed size: {uncompressed_size}, compressed size: {compressed_size}");
        }
    }

    impl PartialEq<Vec<u64>> for CompressedIntsDupes {
        fn eq(&self, other: &Vec<u64>) -> bool {
            self.iter().collect_vec() == *other
        }
    }
}

impl Stones {
    fn new(input: &str) -> Self {
        assert_eq!(input.lines().count(), 1);
        Self(
            input
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    }

    fn blink(&mut self) {
        // let mut new_stones = CompressedIntsDupes::default();
        let mut new_stones = CompressedIntsZstd::default();

        for stone in self.0.iter() {
            if stone == 0 {
                new_stones.push(1);
                continue;
            } else {
                let digits = stone.ilog10() + 1;
                if (digits) % 2 == 0 {
                    let div = 10u64.pow(digits / 2);
                    // println!("split {str} into {a} and {b}");
                    new_stones.push(stone / div);
                    new_stones.push(stone % div);
                    continue;
                }
            }

            new_stones.push(stone.wrapping_mul(2024));
        }

        new_stones.ensure_ready();
        self.0 = new_stones;
    }

    fn blink_n_times(&mut self, n: usize) -> i64 {
        for i in 0..n {
            self.blink();
            println!("blinked {i} times, len is now {}", self.0.len());
            self.0.print_stats();
        }
        self.0.len() as i64
    }
}
