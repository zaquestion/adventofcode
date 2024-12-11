use itertools::Itertools;
use std::{io, iter};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec!["2333133121414131402".to_string()]
    }

    #[rstest]
    #[case(sampledata(), "1928")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "2858")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

fn parse(lines: &Vec<String>) -> Vec<usize> {
    lines
        .first()
        .expect("must first")
        .chars()
        .map(|c| c.to_string().parse::<usize>().expect("must num"))
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> String {
    let nums = parse(lines);
    let files = nums
        .iter()
        .step_by(2)
        .enumerate()
        .flat_map(|(i, n)| iter::repeat(i).take(*n))
        .collect_vec();
    let l = files.len();
    let mut it = files.iter();
    let checksum = nums
        .clone()
        .iter()
        .take(l)
        .enumerate()
        .flat_map(|(i, n)| {
            (0..*n)
                .filter_map(|_| {
                    if i % 2 == 0 {
                        it.next()
                    } else {
                        it.next_back()
                    }
                })
                .collect_vec()
        })
        .enumerate()
        .map(|(i, n)| i * n)
        .sum::<usize>();

    format!("{}", checksum)
}
/*
fn part2(lines: &Vec<String>) -> String {
    let nums = parse(lines);
    // let files = nums.iter().step_by(2).enumerate().collect_vec();

    let files = nums
        .iter()
        .step_by(2)
        .enumerate()
        .flat_map(|(i, n)| iter::repeat(i).take(*n))
        .collect_vec();
    let l = files.len();
    let mut it = files.iter().peekable();
    let revnums = nums.iter().step_by(2).rev();
    let checksum = nums
        .clone()
        .iter()
        .enumerate()
        .flat_map(|(i, n)| {
            if i % 2 == 0 {
                (0..*n).filter_map(|_| it.next())).collect_vec()
            } else {
                if let Some(&f) = revnums.next() {
                    vec![f]
                } else {
                    vec![0 as usize]
                }
            }
        })
        .enumerate()
        .map(|(i, n)| i * n)
        .sum::<usize>();

    format!("{}", checksum)
}
*/

fn build_disk(runs: &Vec<usize>) -> Vec<Option<usize>> {
    // Build the initial disk layout from the runs.
    // Even indices -> file length, assign next file ID
    // Odd indices -> free length, mark free spaces
    let mut disk = Vec::new();
    let mut file_id = 0;
    for (i, &length) in runs.iter().enumerate() {
        if length == 0 {
            // Zero length means no blocks added for this run.
            continue;
        }

        if i % 2 == 0 {
            // File run
            for _ in 0..length {
                disk.push(Some(file_id));
            }
            file_id += 1;
        } else {
            // Free run
            for _ in 0..length {
                disk.push(None);
            }
        }
    }
    disk
}

// Finds a contiguous segment of free (None) blocks of length `size` before `before`.
fn find_free_space(disk: &[Option<usize>], size: usize, before: usize) -> Option<usize> {
    let mut current_start = None;
    let mut current_len = 0;

    for i in 0..before {
        if disk[i].is_none() {
            if current_start.is_none() {
                current_start = Some(i);
                current_len = 1;
            } else {
                current_len += 1;
            }
            if current_len >= size {
                return current_start;
            }
        } else {
            current_start = None;
            current_len = 0;
        }
    }

    None
}

// Move files in descending order of their file ID.
// File ID order is based on the order files appeared initially.
fn move_files_whole(disk: &mut [Option<usize>]) {
    let max_file_id = disk.iter().flatten().max().unwrap_or(&0);
    for fid in (0..=*max_file_id).rev() {
        // Find all positions of this file
        let positions: Vec<usize> = disk
            .iter()
            .enumerate()
            .filter_map(|(idx, &block)| if block == Some(fid) { Some(idx) } else { None })
            .collect();

        if positions.is_empty() {
            continue;
        }

        let file_start = positions[0];
        let file_len = positions.len();

        if let Some(new_start) = find_free_space(disk, file_len, file_start) {
            // Move the file
            // Clear old spots
            for &pos in &positions {
                disk[pos] = None;
            }
            // Place file in new location
            for offset in 0..file_len {
                disk[new_start + offset] = Some(fid);
            }
        }
    }
}

// got a bit weary of solving so decided to see if I could prompt my way to
// part2, and I could, although ironically I had to rewrite the checksome logic
fn part2(lines: &Vec<String>) -> String {
    let runs = parse(lines);
    let mut disk = build_disk(&runs);
    move_files_whole(&mut disk);
    let checksum: usize = disk
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            if let Some(ff) = f {
                Some(i * *ff)
            } else {
                None
            }
        })
        .sum();
    format!("{}", checksum)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
