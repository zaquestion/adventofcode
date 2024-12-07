use itertools::Itertools;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "190: 10 19".to_string(),
            "3267: 81 40 27".to_string(),
            "83: 17 5".to_string(),
            "156: 15 6".to_string(),
            "7290: 6 8 6 15".to_string(),
            "161011: 16 10 13".to_string(),
            "192: 17 8 14".to_string(),
            "21037: 9 7 18 13".to_string(),
            "292: 11 6 16 20".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "3749")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "11387")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Mult,
    Concat,
}

fn parse(lines: &Vec<String>) -> Vec<(usize, Vec<usize>)> {
    lines
        .iter()
        .map(|l| {
            let (target, nums) = l.split(":").collect_tuple().expect("must tuple");
            (
                target.parse::<usize>().expect("must num"),
                nums.split_whitespace()
                    .map(|n| n.parse::<usize>().expect("must num"))
                    .collect_vec(),
            )
        })
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> String {
    let equations = parse(lines);

    let total: usize = equations
        .iter()
        .filter(|(target, nums)| {
            (0..nums.len() - 1)
                .map(|_| [Op::Add, Op::Mult].iter())
                .multi_cartesian_product()
                .find(|path| {
                    path.iter()
                        .zip(nums.iter().skip(1))
                        .fold(nums[0], |acc, (op, n)| match op {
                            Op::Add => acc + n,
                            Op::Mult => acc * n,
                            _ => panic!("no other ops expected"),
                        })
                        == *target
                })
                .is_some()
        })
        .map(|(t, _)| t)
        .sum();

    format!("{}", total)
}

fn part2(lines: &Vec<String>) -> String {
    let equations = parse(lines);

    let total: usize = equations
        .iter()
        .filter(|(target, nums)| {
            (0..nums.len() - 1)
                .map(|_| [Op::Add, Op::Mult, Op::Concat].iter())
                .multi_cartesian_product()
                .find(|path| {
                    path.iter()
                        .zip(nums.iter().skip(1))
                        .fold(nums[0], |acc, (op, n)| match op {
                            Op::Add => acc + n,
                            Op::Mult => acc * n,
                            Op::Concat => {
                                format!("{}{}", acc, n).parse::<usize>().expect("must num")
                            }
                        })
                        == *target
                })
                .is_some()
        })
        .map(|(t, _)| t)
        .sum();

    format!("{}", total)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
