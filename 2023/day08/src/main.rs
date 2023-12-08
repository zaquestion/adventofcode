use futures::executor::block_on;
use futures::future::join_all;
//use futures::join;
use async_std::task;
use itertools::Itertools;
use std::io;
use std::sync::Arc;

// use tokio::task;
fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    // Wrong: 9782823006404936043
    println!("part 2: {:?}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> String {
    let (directions, stepper, _) = parse(lines);

    let (step_count, _) = directions
        .iter()
        .cycle()
        .enumerate()
        .scan("AAA".to_string(), |node, (step, &dir)| {
            let (l, r) = stepper(node.clone());
            *node = match dir {
                'L' => l,
                'R' => r,
                _ => return None,
            };
            Some((step + 1, node.clone()))
        })
        .find(|&(_, ref node)| node == "ZZZ")
        .expect("must find final node");
    return format!("{}", step_count);
}

async fn step(dir: &char, node: &String, stepper: impl Fn(String) -> (String, String)) -> String {
    let (l, r) = stepper(node.to_string());
    let n = match dir {
        'L' => Some(l),
        'R' => Some(r),
        _ => None,
    }
    .expect("must resolve")
    .clone();
    n
}

fn part2(lines: &Vec<String>) -> String {
    let (directions, stepper, nodes) = parse(&lines);

    let arc_stepper = Arc::new(stepper);
    let start_nodes = nodes
        .iter()
        .filter(|n| n.ends_with("A"))
        .map(|n| n.clone())
        .collect_vec();
    let total_end_nodes = start_nodes.len();
    let step_count = directions
        .iter()
        .cycle()
        .enumerate()
        .scan(start_nodes, |nodes, (step_idx, &dir)| {
            let steps = nodes
                .iter()
                .map(|n| {
                    let n_clone = n.clone();
                    let stepper_clone = Arc::clone(&arc_stepper);

                    // async stuffs for the lulz and practice. ghost steps all nodes "at the same time"
                    task::spawn(async move { step(&dir, &n_clone, &*stepper_clone).await })
                })
                .collect_vec();

            *nodes = block_on(async { join_all(steps).await });
            if nodes.iter().any(|n| n.ends_with("Z")) {
                Some((step_idx + 1, true, nodes.clone()))
            } else {
                Some((step_idx + 1, false, nodes.clone()))
            }
        })
        .filter_map(|(step_count, found, _)| if found { Some(step_count) } else { None })
        .take(total_end_nodes)
        .fold(1, |acc, num| {
            let mut a = acc;
            let mut b = num;
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            acc / a * num
        });
    format!("{}", step_count)
}

fn parse(lines: &Vec<String>) -> (Vec<char>, impl Fn(String) -> (String, String), Vec<String>) {
    let mut iter = lines.iter();
    let moves = iter.next().expect("must iter").chars().collect_vec();

    let pathers = iter
        .skip(1)
        .map(|node| {
            let mut split = node.split(" = ");
            let key = split.next().expect("must elem").clone().to_owned();
            let paths = split.next().expect("must elem").clone().to_owned();
            move |v: &String| -> Option<(String, String)> {
                match v {
                    v if &key == v => Some(
                        paths
                            .strip_prefix('(')
                            .unwrap()
                            .strip_suffix(')')
                            .unwrap()
                            .split(",")
                            .map(|s| s.trim().to_string().clone())
                            .collect_tuple::<(String, String)>()
                            .expect("must tupe"),
                    ),
                    _ => None,
                }
            }
        })
        .collect_vec();

    let nodes = lines
        .iter()
        .skip(2)
        .map(|l| l.split(" = ").next().expect("must key").to_string().clone())
        .collect_vec();

    (
        moves,
        move |v: String| pathers.iter().find_map(|f| f(&v)).expect("must resolve"),
        nodes,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("AAA".to_string(), ("BBB".to_string(), "BBB".to_string()))]
    #[case("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string()))]
    #[case("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string()))]
    fn test_pather(#[case] input: String, #[case] expected: (String, String)) {
        let lines = &sampledata();
        let (_, step, _) = parse(lines);
        assert_eq!(expected, step(input));
    }

    fn sampledata() -> Vec<String> {
        vec![
            "LLR".to_string(),
            "".to_string(),
            "AAA = (BBB, BBB)".to_string(),
            "BBB = (AAA, ZZZ)".to_string(),
            "ZZZ = (ZZZ, ZZZ)".to_string(),
        ]
    }

    #[test]
    fn test_part1_sample() -> Result<(), String> {
        assert_eq!("6", part1(&sampledata()));
        Ok(())
    }

    fn sampledata2() -> Vec<String> {
        vec![
            "LR".to_string(),
            "".to_string(),
            "11A = (11B, XXX)".to_string(),
            "11B = (XXX, 11Z)".to_string(),
            "11Z = (11B, XXX)".to_string(),
            "22A = (22B, XXX)".to_string(),
            "22B = (22C, 22C)".to_string(),
            "22C = (22Z, 22Z)".to_string(),
            "22Z = (22B, 22B)".to_string(),
            "XXX = (XXX, XXX)".to_string(),
        ]
    }

    #[test]
    fn test_part2_sample() -> Result<(), String> {
        assert_eq!("6", part2(&sampledata2()));
        Ok(())
    }
}
