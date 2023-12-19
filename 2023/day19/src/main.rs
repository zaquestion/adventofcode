use itertools::Itertools;
use std::{collections::HashMap, io};

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Clone)]
enum WorkflowOutput {
    NewWorkflow(String),
    Accepted,
    Rejected,
}

fn parse(
    lines: &Vec<String>,
) -> (
    HashMap<String, impl Fn(&HashMap<String, usize>) -> WorkflowOutput>,
    Vec<HashMap<String, usize>>,
) {
    let (workflows, parts) = lines
        .split(|l| l.is_empty())
        .collect_tuple()
        .expect("must tuple");
    (
        workflows
            .iter()
            .map(|l| {
                let (wf_name, steps) = l.split('{').collect_tuple().expect("must tuple");
                let transforms =
                    steps
                        .strip_suffix("}")
                        .expect("must }")
                        .split(",")
                        .map(|step| {
                            if let Some((comp, out)) = step.split(":").collect_tuple() {
                                let wfout = match out {
                                    "A" => WorkflowOutput::Accepted,
                                    "R" => WorkflowOutput::Rejected,
                                    _ => WorkflowOutput::NewWorkflow(out.to_string().clone()),
                                };
                                let cat = comp.chars().nth(0).expect("must cat").to_string();
                                let op = comp.chars().nth(1).expect("must op");
                                let num = comp
                                    .chars()
                                    .skip(2)
                                    .collect::<String>()
                                    .parse::<usize>()
                                    .expect("must num");
                                match op {
                            '<' => Some(Box::new(
                                move |hm: &HashMap<String, usize>| -> Option<WorkflowOutput> {
                                    if hm.get(&cat).expect("must hash") < &num {
                                        Some(wfout.clone())
                                    } else {
                                        None
                                    }
                                },
                            )
                                as Box<dyn Fn(&HashMap<String, usize>) -> Option<WorkflowOutput>>),
                            '>' => Some(Box::new(
                                move |hm: &HashMap<String, usize>| -> Option<WorkflowOutput> {
                                    if hm.get(&cat).expect("must hash") > &num {
                                        Some(wfout.clone())
                                    } else {
                                        None
                                    }
                                },
                            )
                                as Box<dyn Fn(&HashMap<String, usize>) -> Option<WorkflowOutput>>),
                            _ => None,
                        }
                        .expect("must resolve")
                            } else {
                                let wfout = match step {
                                    "A" => WorkflowOutput::Accepted,
                                    "R" => WorkflowOutput::Rejected,
                                    _ => WorkflowOutput::NewWorkflow(step.to_string().clone()),
                                };
                                Box::new(
                                    move |hm: &HashMap<String, usize>| -> Option<WorkflowOutput> {
                                        Some(wfout.clone())
                                    },
                                )
                                    as Box<
                                        dyn Fn(&HashMap<String, usize>) -> Option<WorkflowOutput>,
                                    >
                            }
                        })
                        .collect_vec();
                (
                    wf_name.to_string(),
                    move |hm: &HashMap<String, usize>| -> WorkflowOutput {
                        transforms
                            .iter()
                            .find_map(|f| f(hm))
                            .expect("must workflow")
                    },
                )
            })
            .collect(),
        parts
            .iter()
            .map(|l| {
                l.strip_prefix("{")
                    .expect("must {")
                    .strip_suffix("}")
                    .expect("must }")
                    .split(",")
                    .map(|cat_meta| {
                        let (k, v) = cat_meta.split("=").collect_tuple().expect("must tuple");
                        (k.to_string(), v.parse::<usize>().expect("must num"))
                    })
                    .collect::<HashMap<String, usize>>()
            })
            .collect_vec(),
    )
}

fn part1(lines: &Vec<String>) -> String {
    let (workflows, parts) = parse(lines);

    let sum: usize = parts
        .iter()
        .filter_map(|p| {
            let mut wf = "in".to_string();
            loop {
                match workflows.get(&wf).expect("must workflow")(p) {
                    WorkflowOutput::NewWorkflow(new_wf) => {
                        wf = new_wf;
                    }
                    WorkflowOutput::Accepted => return Some(p),
                    WorkflowOutput::Rejected => return None,
                }
            }
        })
        .map(|p| p.iter().map(|(_, v)| v).sum::<usize>())
        .sum();

    format!("{}", sum)
}

fn parse_pt2(lines: &Vec<String>) -> HashMap<String, (Vec<(char, bool, usize, String)>, String)> {
    let (workflows, _) = lines
        .split(|l| l.is_empty())
        .collect_tuple()
        .expect("must tuple");

    workflows
        .iter()
        .map(|l| {
            let (wf_name, remaining) = l.split('{').collect_tuple().expect("must tuple");
            let steps = remaining
                .strip_suffix("}")
                .expect("must }")
                .split(",")
                .collect_vec();
            (
                wf_name.to_string(),
                (
                    steps[..steps.len() - 1]
                        .iter()
                        .map(|s| {
                            s.split(":")
                                .tuples()
                                .map(|(comp, out)| {
                                    (
                                        comp.chars().nth(0).expect("must cat"),
                                        comp.chars().nth(1).expect("must op") == '<',
                                        comp.chars()
                                            .skip(2)
                                            .collect::<String>()
                                            .parse::<usize>()
                                            .expect("must num"),
                                        out.to_string(),
                                    )
                                })
                                .exactly_one()
                                .expect("must one")
                        })
                        .collect_vec(),
                    steps[steps.len() - 1].to_string(),
                ),
            )
        })
        .collect()
}

fn part2(lines: &Vec<String>) -> String {
    let workflows = parse_pt2(lines);

    let ranges = "xmas"
        .chars()
        .map(|c| (c, (1, 4000)))
        .collect::<HashMap<char, (usize, usize)>>();

    let n_possibilities = count_acceptable(&workflows, "in".to_string(), ranges);

    format!("{}", n_possibilities)
}

fn count_acceptable(
    workflows: &HashMap<String, (Vec<(char, bool, usize, String)>, String)>,
    wf_name: String,
    mut ranges: HashMap<char, (usize, usize)>,
) -> usize {
    if wf_name == "A".to_string() {
        return ranges.iter().map(|(_, (lo, hi))| hi - lo + 1).product();
    }
    if wf_name == "R".to_string() {
        return 0;
    }
    let mut ans = 0;
    let (rules, fallback) = workflows.get(&wf_name).expect("must workflow");
    for (c, lt, n, new_wf) in rules {
        let (lo, hi) = ranges.get(&c).expect("must range");
        let mut newranges = ranges.clone();
        *newranges.get_mut(&c).expect("must get c") = if *lt { (*lo, n - 1) } else { (n + 1, *hi) };
        ans += count_acceptable(workflows, new_wf.clone(), newranges);
        *ranges.get_mut(&c).expect("must get c") = if *lt { (*n, *hi) } else { (*lo, *n) };
    }
    ans += count_acceptable(workflows, fallback.clone(), ranges);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "px{a<2006:qkq,m>2090:A,rfg}".to_string(),
            "pv{a>1716:R,A}".to_string(),
            "lnx{m>1548:A,A}".to_string(),
            "rfg{s<537:gd,x>2440:R,A}".to_string(),
            "qs{s>3448:A,lnx}".to_string(),
            "qkq{x<1416:A,crn}".to_string(),
            "crn{x>2662:A,R}".to_string(),
            "in{s<1351:px,qqz}".to_string(),
            "qqz{s>2770:qs,m<1801:hdj,R}".to_string(),
            "gd{a>3333:R,R}".to_string(),
            "hdj{m>838:A,pv}".to_string(),
            "".to_string(),
            "{x=787,m=2655,a=1222,s=2876}".to_string(),
            "{x=1679,m=44,a=2067,s=496}".to_string(),
            "{x=2036,m=264,a=79,s=2244}".to_string(),
            "{x=2461,m=1339,a=466,s=291}".to_string(),
            "{x=2127,m=1623,a=2188,s=1013}".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "19114")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "167409079868000")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}
