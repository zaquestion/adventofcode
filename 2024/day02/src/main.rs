use itertools::Itertools;
use std::{io, ops::Index};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "2")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "4")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
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

fn part1(lines: &Vec<String>) -> String {
    let reports = parse(lines);
    let num_safe = reports
        .iter()
        .filter(|report| {
            report
                .iter()
                .tuple_windows()
                .all(|(a, b)| a.abs_diff(*b) <= 3 && a < b)
        })
        .count();
    let num_safe_inc = reports
        .iter()
        .filter(|report| {
            report
                .iter()
                .tuple_windows()
                .all(|(a, b)| a.abs_diff(*b) <= 3 && a > b)
        })
        .count();

    format!("{}", num_safe + num_safe_inc)
}

fn part2(lines: &Vec<String>) -> String {
    let reports = parse(lines);

    let num_safe = reports
        .iter()
        .filter(|report| {
            report.iter().enumerate().any(|(i, _)| {
                let mut modified_report = report.to_vec();
                modified_report.remove(i);

                let is_increasing = modified_report
                    .iter()
                    .tuple_windows()
                    .all(|(a, b)| a.abs_diff(*b) <= 3 && a < b);

                let is_decreasing = modified_report
                    .iter()
                    .tuple_windows()
                    .all(|(a, b)| a.abs_diff(*b) <= 3 && a > b);

                is_increasing || is_decreasing
            })
        })
        .count();

    format!("{}", num_safe)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
