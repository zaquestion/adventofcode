// use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io,
    mem::swap,
};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sample_data() -> Vec<String> {
        vec![
            "broadcaster -> a, b, c".to_string(),
            "%a -> b".to_string(),
            "%b -> c".to_string(),
            "%c -> inv".to_string(),
            "&inv -> a".to_string(),
        ]
    }

    fn sample_2_data() -> Vec<String> {
        vec![
            "broadcaster -> a".to_string(),
            "%a -> inv, con".to_string(),
            "&inv -> b".to_string(),
            "%b -> con".to_string(),
            "&con -> output".to_string(),
        ]
    }

    #[rstest]
    #[case(sample_data(), "32000000")]
    #[case(sample_2_data(), "11687500")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    // no sample input for part 2
}

#[derive(Debug, PartialEq, Clone)]
enum ModuleType {
    BroadCaster,
    FlipFlop(bool),
    Conjunction((HashSet<String>, HashSet<String>)),
}

#[derive(Debug, PartialEq, Clone)]
struct Module {
    module_type: ModuleType,
    outputs: Vec<String>,
}

fn parse(lines: &Vec<String>) -> HashMap<String, Module> {
    lines
        .iter()
        .map(|l| {
            l.split(" -> ")
                .tuples()
                .map(|(m, output_str)| {
                    let outputs = output_str.split(", ").map(|s| s.to_string()).collect_vec();
                    match m.chars().nth(0).expect("must first") {
                        '%' => (
                            m.strip_prefix("%").expect("must % prefix").to_string(),
                            Module {
                                module_type: ModuleType::FlipFlop(false),
                                outputs: outputs,
                            },
                        ),
                        '&' => {
                            let name = m.strip_prefix("&").expect("must & prefix").to_string();
                            (
                                name.clone(),
                                Module {
                                    module_type: ModuleType::Conjunction((
                                        lines
                                            .iter()
                                            .flat_map(|l| {
                                                l.split(" -> ").tuples().filter_map(
                                                    |(m_2, outs)| {
                                                        if outs.contains(&name) {
                                                            Some(
                                                                m_2.trim_start_matches(['&', '%'])
                                                                    .to_string(),
                                                            )
                                                        } else {
                                                            None
                                                        }
                                                    },
                                                )
                                            })
                                            .collect(),
                                        HashSet::new(),
                                    )),
                                    outputs: outputs,
                                },
                            )
                        }
                        'b' => (
                            m.to_string(),
                            Module {
                                module_type: ModuleType::BroadCaster,
                                outputs: outputs,
                            },
                        ),
                        _ => panic!("unknown module"),
                    }
                })
                .exactly_one()
                .expect("must one")
        })
        .collect::<HashMap<String, Module>>()
}

fn part1(lines: &Vec<String>) -> String {
    let modules = &mut parse(lines);

    let sums = (0..1000).fold((0, 0), |(low, high), _| {
        let (l, h, _) = activate_redstone(modules);
        (low + l, high + h)
    });

    format!("{}", sums.0 * sums.1)
}

fn part2(lines: &Vec<String>) -> String {
    let modules = &mut parse(lines);

    let presses = (1..usize::MAX)
        .filter_map(|i| {
            let (_, _, cl_senders) = activate_redstone(modules);
            if cl_senders.is_empty() {
                None
            } else {
                Some(
                    cl_senders
                        .iter()
                        .map(|s| (i.clone(), s.clone()))
                        .collect_vec(),
                )
            }
        })
        .flatten()
        .unique_by(|(_, s)| s.clone())
        .take(4) // my input has 4 "cl"
        .fold(1, |acc, (num, _)| {
            let mut a = acc;
            let mut b = num;
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            acc / a * num
        });

    format!("{}", presses)
}

type Signal = (String, String, bool);

fn activate_redstone(modules: &mut HashMap<String, Module>) -> (usize, usize, HashSet<String>) {
    // let mut high: HashSet<String> = HashSet::new();
    // let mut low: HashSet<String> = HashSet::new();

    let mut signals: Vec<Signal> = vec![("button".to_string(), "broadcaster".to_string(), false)];
    let mut cl_senders: HashSet<String> = HashSet::new();
    let (mut low_signals, mut high_signals) = (1, 0);
    while !signals.is_empty() {
        // dbg!(&signals);
        let mut newsignals = signals
            .drain(0..)
            .flat_map(|(sender, name, is_high_pulse)| {
                if let Some(m) = modules.get_mut(&name) {
                    match &mut m.module_type {
                        ModuleType::BroadCaster => m
                            .outputs
                            .iter()
                            .map(|o| (name.clone(), o.clone(), is_high_pulse))
                            .collect_vec(),
                        ModuleType::FlipFlop(on) if !is_high_pulse => {
                            let sigs = m
                                .outputs
                                .iter()
                                .map(|o| (name.clone(), o.clone(), !*on))
                                .collect_vec();
                            *on = !*on;
                            sigs
                        }
                        ModuleType::FlipFlop(_) => vec![],
                        ModuleType::Conjunction((inputs, high_signals)) => {
                            if !inputs.contains(&sender) {
                                panic!("bad sender");
                            }
                            if is_high_pulse {
                                high_signals.insert(sender);
                            } else {
                                high_signals.remove(&sender);
                            }
                            m.outputs
                                .iter() // high pulse unless all inputs had high signals
                                .map(|o| {
                                    (name.clone(), o.clone(), inputs.len() != high_signals.len())
                                })
                                .collect_vec()
                        }
                    }
                } else {
                    vec![]
                }
            })
            .collect_vec();
        cl_senders.extend(
            newsignals
                .iter()
                .filter_map(|(sender, name, is_high)| {
                    if name == "cl" && *is_high {
                        Some(sender.clone())
                    } else {
                        None
                    }
                })
                .into_iter(),
        );
        let newcounts = newsignals.iter().map(|(_, _, high)| *high).counts();
        // dbg!(&newsignals);
        low_signals += newcounts.get(&false).unwrap_or(&0);
        high_signals += newcounts.get(&true).unwrap_or(&0);
        swap(&mut signals, &mut newsignals);
    }
    (low_signals, high_signals, cl_senders)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    // WRONG: 31297135578282
    println!("part 2: {:?}", part2(&lines));
}
