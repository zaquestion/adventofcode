use indicatif::ProgressIterator;
use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> String {
    let (times, dists) = parse(lines);

    let prod: usize = times
        .iter()
        .map(|&t| (1..t).map(|i| (t - i) * i).collect_vec())
        .enumerate()
        .map(|(i, opt)| opt.iter().filter(|&o| o > &dists[i]).count())
        .product();

    format!("{}", prod)
}

fn part2(lines: &Vec<String>) -> String {
    let (big_race_time, big_race_dist) = dbg!(parse2(lines));

    let prod: usize = (0..big_race_time)
        .progress()
        .map(|i| (big_race_time - i) * i)
        .filter(|&o| o > big_race_dist)
        .count();

    format!("{}", prod)
}

fn parse(lines: &Vec<String>) -> (Vec<usize>, Vec<usize>) {
    (
        lines[0]
            .strip_prefix("Time:")
            .expect("must split")
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("must num"))
            .collect_vec(),
        lines[1]
            .strip_prefix("Distance:")
            .expect("must split")
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("must num"))
            .collect_vec(),
    )
}

fn parse2(lines: &Vec<String>) -> (usize, usize) {
    (
        lines[0]
            .strip_prefix("Time:")
            .expect("must split")
            .replace(" ", "")
            .parse::<usize>()
            .expect("must num"),
        lines[1]
            .strip_prefix("Distance:")
            .expect("must split")
            .replace(" ", "")
            .parse::<usize>()
            .expect("must num"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sampledata() -> Vec<String> {
        vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ]
    }

    #[test]
    fn test_part1_sample() -> Result<(), String> {
        assert_eq!("288", part1(&sampledata()));
        Ok(())
    }

    #[test]
    fn test_part2_sample() -> Result<(), String> {
        assert_eq!("71503", part2(&sampledata()));
        Ok(())
    }
}
