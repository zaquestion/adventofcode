use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines[0]));
    println!("part 2: {:?}", part2(&lines[0]));
}

fn hash(input: &String) -> usize {
    input
        .chars()
        .map(|c| c as usize)
        .fold(0, |acc, ascii| (acc + ascii) * 17 % 256)
}

fn part1(line: &String) -> String {
    // let board = parse(lines);

    let sum: usize = line.split(',').map(|str| hash(&str.to_string())).sum();

    format!("{}", sum)
}

fn part2(line: &String) -> String {
    let ops = parse(line);

    let boxes = ops.iter().fold(vec![Vec::new(); 256], |mut boxes, op| {
        let h = hash(&op.0);
        match op.1 {
            Operation::Equal(_) => {
                if let Some((i, _)) = boxes[h]
                    .iter()
                    .enumerate()
                    .find(|(_, (label, _))| label == &op.0)
                {
                    boxes[h][i] = op.to_owned();
                } else {
                    boxes[h].push(op.to_owned());
                }
            }
            Operation::Dash => {
                if let Some((i, _)) = boxes[h]
                    .iter()
                    .enumerate()
                    .find(|(_, (label, _))| label == &op.0)
                {
                    boxes[h].remove(i);
                }
            }
        }
        boxes
    });

    let sum: usize = boxes
        .iter()
        .enumerate()
        .flat_map(|(box_n, b)| {
            b.iter()
                .map(|lens| {
                    match lens.1 {
                        Operation::Equal(f) => Some(f),
                        _ => None,
                    }
                    .expect("must resolve")
                })
                .enumerate()
                .map(|(i, focal_len)| (box_n + 1) * (i + 1) * focal_len)
                .collect_vec()
        })
        .sum();

    format!("{}", sum)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Dash,
    Equal(usize),
}

fn parse(line: &String) -> Vec<(String, Operation)> {
    line.split(',')
        .map(|step| {
            if step.chars().last().expect("must last") == '-' {
                Some((
                    step.strip_suffix('-').expect("must strip").to_string(),
                    Operation::Dash,
                ))
            } else if step.contains('=') {
                Some(
                    step.split("=")
                        .tuples()
                        .map(|(l, r)| {
                            (
                                l.to_string(),
                                Operation::Equal(r.parse::<usize>().expect("must num")),
                            )
                        })
                        .exactly_one()
                        .expect("must one"),
                )
            } else {
                None
            }
            .expect("must resolve")
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", "52")]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", "1320")]
    fn test_part1_sample(#[case] input: String, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", "145")]
    fn test_part2_sample(#[case] input: String, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}
