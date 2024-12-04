use itertools::Itertools;
use regex::Regex;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string()]
    }

    #[rstest]
    #[case(sampledata(), "161")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    fn sample2data() -> Vec<String> {
        vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ]
    }

    #[rstest]
    #[case(sample2data(), "48")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Mul(isize, isize),
    Do,
    Dont,
}

fn parse(lines: &Vec<String>) -> Vec<Instruction> {
    let op_str = lines.concat();
    let mul_re = Regex::new(r"(mul|do|don\'t)\(([0-9]+)?,?([0-9]+)?\)").unwrap();
    mul_re
        .captures_iter(op_str.as_str())
        .map(|c| (c.get(1).expect("must op").as_str(), c))
        .filter_map(|(t, c)| match t {
            "mul" => {
                if let (Some(x), Some(y)) = (c.get(2), c.get(3)) {
                    Some(Instruction::Mul(
                        x.as_str().parse::<isize>().expect("must num"),
                        y.as_str().parse::<isize>().expect("must num"),
                    ))
                } else {
                    None
                }
            }

            "do" => Some(Instruction::Do),
            "don't" => Some(Instruction::Dont),
            _ => panic!("{}", t),
        })
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> String {
    let ops = parse(lines);
    let sum = ops.iter().fold(0, |acc, op| {
        if let Instruction::Mul(x, y) = op {
            acc + x * y
        } else {
            0
        }
    });

    format!("{}", sum)
}

fn part2(lines: &Vec<String>) -> String {
    let ops = parse(lines);
    let sum = ops
        .iter()
        .fold((0, Instruction::Do), |(acc, last), op| match op {
            Instruction::Mul(x, y) => {
                if last == Instruction::Do {
                    (acc + x * y, last)
                } else {
                    (acc, last)
                }
            }
            Instruction::Do | Instruction::Dont => (acc, op.clone()),
        })
        .0;

    format!("{}", sum)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
