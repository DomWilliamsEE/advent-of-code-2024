use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat_n;
use std::process::id;

// pub const PART1_EXAMPLE: &str = "2333133121414131402";
pub const PART1_EXAMPLE: &str = "12345";
// leave blank if same as part1
pub const PART2_EXAMPLE: &str = "";

#[derive(Debug)]
struct Block {
    file: Option<i32>,
}

struct DiskMap {
    blocks: Vec<Block>,
}

// impl ToString for DiskMap {
fn fmt(blocks: &[Block]) -> String {
    let mut s = String::new();

    for block in blocks {
        let char = match block.file {
            None => '.',
            Some(id) => id.to_string().chars().next().unwrap(),
        };

        s.push(char);
    }

    s
}

fn compact(input: &str) -> Vec<Block> {
    let mut nums = input.chars().map(|c| c.to_digit(10).unwrap());
    let mut next_is_file = true;
    let mut next_file_id = 0;

    let mut blocks = nums
        .flat_map(|len| {
            let ty = if next_is_file {
                let id = next_file_id;
                next_file_id += 1;
                Some(id)
            } else {
                None
            };
            next_is_file = !next_is_file;

            repeat_n(ty, len as usize).map(|file| Block { file })
        })
        .collect_vec();

    let gaps = {
        let end_idx = blocks.iter().rposition(|b| b.file.is_none()).unwrap();
        blocks
            .iter()
            .take(end_idx + 1)
            .filter(|b| b.file.is_none())
            .count()
    };

    let mut next_start = 0;
    for _ in 0..gaps {
        // println!(
        //     "{}",
        //     blocks
        //         .iter()
        //         .map(|b| b
        //             .file
        //             .map(|i| i.to_string().chars().next().unwrap())
        //             .unwrap_or('.'))
        //         .join("")
        // );

        let last_block = blocks.iter().rposition(|b| b.file.is_some()).unwrap();
        let free = blocks
            .iter()
            .skip(next_start)
            .position(|b| b.file.is_none())
            .unwrap()
            + next_start;

        if last_block < free {
            break; // already compacted?
        }

        // dbg!(free, last_block);
        next_start = free + 1;

        blocks.swap(free, last_block);
    }

    blocks
}

fn compact_whole_files(input: &str) -> Vec<Block> {
    let mut nums = input.chars().map(|c| c.to_digit(10).unwrap());
    let mut next_is_file = true;
    let mut next_file_id = 0;

    let mut file_lens = HashMap::new();
    let mut blocks = nums
        .flat_map(|len| {
            let ty = if next_is_file {
                let id = next_file_id;
                file_lens.insert(id, len as usize);
                next_file_id += 1;
                Some(id)
            } else {
                None
            };
            next_is_file = !next_is_file;

            repeat_n(ty, len as usize).map(|file| Block { file })
        })
        .collect_vec();

    let file_indices = (0..next_file_id).rev();
    dbg!(&file_indices);
    for file_to_move in file_indices {
        let len = file_lens[&file_to_move];

        let move_to_idx = {
            blocks
                .windows(len)
                .enumerate()
                .find_map(|(i, window)| window.iter().all(|b| b.file.is_none()).then_some(i))
        };

        let orig_file_location = blocks
            .iter()
            .position(|b| b.file == Some(file_to_move))
            .unwrap();

        println!("move file {file_to_move} of len {len} to {move_to_idx:?} from {orig_file_location}");

        if let Some(move_to_idx) = move_to_idx {
            if move_to_idx < orig_file_location {
                blocks
                    .iter_mut()
                    .skip(move_to_idx)
                    .take(len)
                    .for_each(|b| b.file = Some(file_to_move));

                blocks
                    .iter_mut()
                    .skip(orig_file_location)
                    .take(len)
                    .for_each(|b| b.file = None);
            }
        }

        // println!("{}", fmt(&blocks));
    }

    blocks
}

fn checksum(fs: &[Block]) -> i64 {
    fs.iter()
        .enumerate()
        .map(|(i, b)| b.file.map(|f| i as i64 * f as i64).unwrap_or(0))
        .sum()
}

pub fn part_one(input: &str) -> i64 {
    assert_eq!(input.lines().count(), 1);

    let s = compact(input.trim());
    checksum(&s)
}

pub fn part_two(input: &str) -> i64 {
    assert_eq!(input.lines().count(), 1);

    let s = compact_whole_files(input.trim());
    checksum(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let s = compact("12345");
        println!("{}", fmt(&s));
        // assert_eq!(checksum(&s), 1928);
    }

    #[test]
    fn test_example2() {
        let s = compact("2333133121414131402");
        println!("{}", fmt(&s));
        assert_eq!(checksum(&s), 1928);
    }

    #[test]
    fn test_example2_part2() {
        let s = compact_whole_files("2333133121414131402");
        println!("{}", fmt(&s));
        assert_eq!(checksum(&s), 2858);
    }
}
