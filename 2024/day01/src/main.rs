use itertools::Itertools;
use std::collections::HashMap;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "11")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "31")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

fn parse(lines: &Vec<String>) -> (Vec<isize>, Vec<isize>) {
    (
        lines
            .iter()
            .map(|l| {
                l.split_whitespace()
                    .take(1)
                    .map(|s| s.parse::<isize>().expect("must num"))
                    .exactly_one()
                    .expect("must one")
            })
            .collect_vec(),
        lines
            .iter()
            .map(|l| {
                l.split_whitespace()
                    .skip(1)
                    .take(1)
                    .map(|s| s.parse::<isize>().expect("must num"))
                    .exactly_one()
                    .expect("must one")
            })
            .collect_vec(),
    )
}

fn part1(lines: &Vec<String>) -> String {
    let (mut a, mut b) = parse(lines);
    a.sort();
    b.sort();
    let ans: isize = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| a.abs_diff(*b) as isize)
        .sum();

    format!("{}", ans)
}

fn part2(lines: &Vec<String>) -> String {
    let (a, b) = parse(lines);

    // I didn't even need this one oopsie
    //
    // let mut hash_a: HashMap<isize, usize> = HashMap::new();
    // for n in a.iter() {
    //     if let Some(val) = hash_a.get(n) {
    //         hash_a.insert(n.clone(), val + 1);
    //     } else {
    //         hash_a.insert(n.clone(), 1);
    //     }
    // }
    let mut hash_b: HashMap<isize, usize> = HashMap::new();
    for n in b.iter() {
        if let Some(val) = hash_b.get(n) {
            hash_b.insert(n.clone(), val + 1);
        } else {
            hash_b.insert(n.clone(), 1);
        }
    }
    let sum: usize = a
        .iter()
        .map(|k| *k as usize * hash_b.get(&k).map(|v| v.clone()).unwrap_or_default())
        .sum();

    format!("{}", sum)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
