use itertools::Itertools;
use std::cmp::Ordering;
use std::{collections::HashMap, collections::HashSet, io};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "47|53".to_string(),
            "97|13".to_string(),
            "97|61".to_string(),
            "97|47".to_string(),
            "75|29".to_string(),
            "61|13".to_string(),
            "75|53".to_string(),
            "29|13".to_string(),
            "97|29".to_string(),
            "53|29".to_string(),
            "61|53".to_string(),
            "97|53".to_string(),
            "61|29".to_string(),
            "47|13".to_string(),
            "75|47".to_string(),
            "97|75".to_string(),
            "47|61".to_string(),
            "75|61".to_string(),
            "47|29".to_string(),
            "75|13".to_string(),
            "53|13".to_string(),
            "".to_string(),
            "75,47,61,53,29".to_string(),
            "97,61,53,29,13".to_string(),
            "75,29,13".to_string(),
            "75,97,47,61,53".to_string(),
            "61,13,29".to_string(),
            "97,13,75,29,47".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "143")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "123")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

fn parse(lines: &Vec<String>) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let (rule_str, updates) = lines
        .split(|l| l == "")
        .collect_tuple()
        .expect("must tuple");
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    rule_str.iter().take_while(|l| *l != "").for_each(|l| {
        let (l, r) = l
            .split("|")
            .map(|nums| nums.parse::<usize>().expect("must num"))
            .collect_tuple()
            .expect("must tuple");
        if let Some(befores) = rules.get_mut(&l) {
            befores.insert(r);
        } else {
            rules.insert(l, HashSet::from([r]));
        }
    });
    (
        rules,
        updates
            .iter()
            .map(|l| {
                l.split(",")
                    .map(|n| n.parse::<usize>().expect("must num"))
                    .collect_vec()
            })
            .collect_vec(),
    )
}

fn part1(lines: &Vec<String>) -> String {
    let (rules, updates) = parse(lines);
    let sum = updates
        .iter()
        .map(|u| {
            let mut sorted = u.clone();
            sorted.sort_by(|a, b| {
                if let Some(befores) = rules.get(a) {
                    if befores.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Equal
                }
            });
            if u == &sorted {
                u[u.len() / 2]
            } else {
                0
            }
        })
        .sum::<usize>();

    format!("{}", sum)
}

fn part2(lines: &Vec<String>) -> String {
    let (rules, updates) = parse(lines);
    let sum = updates
        .iter()
        .map(|u| {
            let mut sorted = u.clone();
            sorted.sort_by(|a, b| {
                if let Some(befores) = rules.get(a) {
                    if befores.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                } else {
                    Ordering::Equal
                }
            });
            if u == &sorted {
                0
            } else {
                sorted[u.len() / 2]
            }
        })
        .sum::<usize>();

    format!("{}", sum)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
