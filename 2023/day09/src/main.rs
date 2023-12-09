use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> String {
    let seqs = parse(lines);

    let deltas = seqs
        .iter()
        .map(|seq| {
            let mut deltavec: Vec<Vec<isize>> = Vec::new();
            deltavec.push(seq.clone());
            while !deltavec.last().expect("must last").iter().all(|&d| d == 0) {
                deltavec.push(
                    deltavec
                        .last()
                        .expect("must elem")
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect_vec()
                        .clone(),
                );
            }
            deltavec
        })
        .collect_vec();
    let deltasum: isize = deltas
        .iter()
        .map(|dl| {
            dl.iter()
                .rev()
                .map(|d| d.last().expect("must elem"))
                .sum::<isize>()
        })
        .sum();

    format!("{}", deltasum)
}

fn part2(lines: &Vec<String>) -> String {
    let seqs = parse(lines);

    let deltas = seqs
        .iter()
        .map(|seq| {
            let mut deltavec: Vec<Vec<isize>> = Vec::new();
            deltavec.push(seq.clone());
            while !deltavec.last().expect("must last").iter().all(|&d| d == 0) {
                deltavec.push(
                    deltavec
                        .last()
                        .expect("must elem")
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect_vec()
                        .clone(),
                );
            }
            deltavec
        })
        .collect_vec();
    let deltasum: isize = deltas
        .iter()
        .map(|dl| {
            dl.iter()
                .rev()
                .map(|d| d.first().expect("must elem"))
                .fold(0isize, |acc, n| n - acc)
        })
        .sum();

    format!("{}", deltasum)
}

fn parse(lines: &Vec<String>) -> Vec<Vec<isize>> {
    lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse::<isize>().expect("must num"))
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sampledata() -> Vec<String> {
        vec![
            "0 3 6 9 12 15".to_string(),
            "1 3 6 10 15 21".to_string(),
            "10 13 16 21 30 45".to_string(),
        ]
    }

    #[test]
    fn test_part1_sample() -> Result<(), String> {
        assert_eq!("114", part1(&sampledata()));
        Ok(())
    }

    #[test]
    fn test_part2_sample() -> Result<(), String> {
        assert_eq!("2", part2(&sampledata()));
        Ok(())
    }
}
