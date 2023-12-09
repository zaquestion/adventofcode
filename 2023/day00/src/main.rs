use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
}

fn part1(lines: &Vec<String>) -> String {
    let board = parse(lines);

    format!("{}", "todo")
}

fn part2(lines: &Vec<String>) -> String {
    let board = parse(lines);

    format!("{}", "todo")
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Cell>> {}

#[cfg(test)]
mod tests {
    use super::*;
    fn sampledata() -> Vec<String> {
        vec![]
    }

    #[test]
    fn test_part1_sample() -> Result<(), String> {
        assert_eq!("something", part1(&sampledata()));
        Ok(())
    }

    #[test]
    fn test_part2_sample() -> Result<(), String> {
        assert_eq!("something", part2(&sampledata()));
        Ok(())
    }
}
