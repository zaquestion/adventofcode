use itertools::Itertools;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![]
    }

    #[rstest]
    #[case(sampledata(), "unexpected")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "unexpected")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Cell>> {
    todo!()
}

fn part1(lines: &Vec<String>) -> String {
    let board = parse(lines);

    format!("{}", todo!())
}

fn part2(lines: &Vec<String>) -> String {
    let board = parse(lines);

    format!("{}", todo!())
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
