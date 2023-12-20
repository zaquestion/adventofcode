// #![feature(test)]

use async_std::task;
use futures::executor::block_on;
use futures::future::join_all;
use itertools::Itertools;
use std::collections::HashMap;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn parse(lines: &Vec<String>) -> Vec<(Vec<Spring>, Vec<usize>)> {
    lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .tuples()
                .map(|(l, r)| {
                    (
                        l.chars()
                            .map(|c| {
                                match c {
                                    '.' => Some(Spring::Operational),
                                    '#' => Some(Spring::Damaged),
                                    '?' => Some(Spring::Unknown),
                                    _ => None,
                                }
                                .expect("must resolve")
                            })
                            .collect_vec(),
                        r.split(",")
                            .map(|c| c.parse::<usize>().expect("must num"))
                            .collect_vec(),
                    )
                })
                .exactly_one()
                .expect("must one")
        })
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> String {
    let records = parse(lines);

    let sum: usize = records
        .iter()
        .map(|r| process_record_fast(&r.clone()))
        // .inspect(|s| {
        //     dbg!(s);
        // })
        .sum();

    format!("{}", sum)
}

fn process_record(record: &(Vec<Spring>, Vec<usize>)) -> usize {
    let (arrangement, damaged_counts) = record;
    let spring_counts = arrangement.iter().counts_by(|c| c);
    let total_damaged: usize = damaged_counts.iter().sum();
    let unknown = spring_counts.get(&Spring::Unknown).expect("must counted");

    // let combos = (0..unknown_damaged)
    //     .map(|_| Spring::Damaged)
    //     .chain((0..unknown_working).map(|_| Spring::Operational))
    //     .permutations(*unknown)
    //     .collect_vec();
    // dbg!(combos.len());
    //
    // The above produced wayyyy to many options, so using
    // multi_cartesian_product instead allowed the candidate set to be a lot
    // smaller

    (0..*unknown)
        .map(|_| vec![Spring::Operational, Spring::Damaged])
        .multi_cartesian_product()
        // dbg!(combos2.len());
        // dbg!(damaged_counts);
        .map(|combo| {
            let mut iter = combo.iter();
            arrangement
                .iter()
                .map(|spring| {
                    if *spring == Spring::Unknown {
                        iter.next().expect("must spring").clone()
                    } else {
                        *spring
                    }
                })
                .collect_vec()
        })
        .filter(|opt| {
            let grps = opt
                .iter()
                .group_by(|&c| c)
                .into_iter()
                .filter(|(&k, _)| k == Spring::Damaged)
                .map(|(_, g)| g.count())
                .collect_vec();

            grps.len() == damaged_counts.len()
                && grps.iter().zip(damaged_counts).all(|(l, r)| *l == *r)
        })
        // .inspect(|s| {
        //     s.iter().for_each(|c| {
        //         print!(
        //             "{}",
        //             match c {
        //                 Spring::Operational => Some("."),
        //                 Spring::Damaged => Some("#"),
        //                 _ => None,
        //             }
        //             .expect("must resolve")
        //         )
        //     });
        //     println!();
        // })
        .count()
}

// fn process_record_recursive(record: &(Vec<Spring>, Vec<usize>)) -> usize {}

fn process_record_fast(record: &(Vec<Spring>, Vec<usize>)) -> usize {
    // really helpful videos for helping me grok DynProg
    // https://www.youtube.com/watch?v=Hdr64lKQ3e4

    /*
        // big ass notes to help me reason about the propgation of base cases

        //  vec!["???.### 1,1,3".to_string()]
        // in iter order (rev of vec order)
                    0                           1                           2                           3
        initial    [[ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 1, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0 ]]
                                                                                    v--^                  v-in group no copy
        pos = 6, # [[ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 1, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ]]
                                                                                 v--^
        pos = 5, # [[ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 1, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ]]
                                                                              v--^ propagation of contiguous damaged counts, reach 0th idx means satisfying the group
        pos = 4, # [[ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ]]
                                                     v------------------------^ "." seen add over any permutations from the previous group into idx for the start of the next
        pos = 3, . [[ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 1, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ]]
                                                  +--^ add the previous count and the count from the last position for group reset
                                                  v  v------------------------^ props value for "." to add new groups
        pos = 2, ? [[ 0, 0, 0, 0, 0, 0, 0, 0 ], [ 1, 1, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ]]
                         v------------------------^ propagate completed group into start idx of the next
                         v                        v--v adds for both "." and "#" continue
        pos = 1, ? [[ 0, 1, 0, 0, 0, 0, 0, 0 ], [ 2, 1, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ]]
                      v--^ add over count for final group completing
        pos = 0, ? [[ 1, 2, 0, 0, 0, 0, 0, 0 ], [ 3, 1, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0 ]]



        // simplified case 4
        vec!["????.#.#. 4,1,1".to_string()]
                    0                                 1                                 2                                 3
        initial    [[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
                                                                                             v - prop for new group         v - copy because not in group
        pos = 9, . [[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
                                                                                          v--^ found "#"
        pos = 8, # [[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
                                                           v------------------------------^ prop completed group
                                                           v                              v - prop because "."
        pos = 7, . [[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 1, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
                                                        v--^ found "#"
        pos = 6, # [[ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
                                  v---------------------^ prop completed group
                                  v                     v - prop because "."
        pos = 5, . [[ 0, 0, 0, 0, 1, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
                               v--^ prop possible "#"
                               v  v prop "."            v - prop because "."
        pos = 4, ? [[ 0, 0, 0, 1, 1, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
                               above continues, with each possible "#" from the previous step being propagated by all possible "."
        pos = 3, ? [[ 0, 0, 1, 1, 1, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
        pos = 2, ? [[ 0, 1, 1, 1, 1, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
        pos = 1, ? [[ 1, 1, 1, 1, 1, 0, 0, 0, 0, 0 ], [ 1, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ], [ 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ]]
    */

    let (arrangement, damaged_counts) = record;
    let mut memo: Vec<Vec<Vec<usize>>> =
        vec![vec![vec![0; arrangement.len() + 1]; damaged_counts.len() + 1]; arrangement.len() + 1];
    let max_groups = damaged_counts.len();

    // propagate from having no contiguous damaged cells where the last (first
    // from the algos perspective) group. This value will prop into finding the initial
    // group in the algo
    memo[arrangement.len()][max_groups][0] = 1;

    // handles propagating when the lines end in damaged or unknown cells
    // from the perspective of the algorithm the counting begins immediately
    memo[arrangement.len()][max_groups - 1][damaged_counts[max_groups - 1]] = 1;

    for pos in (0..arrangement.len()).rev() {
        for group in 0..damaged_counts.len() {
            for damaged in 0..=damaged_counts[group] {
                for opt in [Spring::Operational, Spring::Damaged].iter() {
                    if matches!(arrangement[pos], Spring::Damaged | Spring::Unknown)
                        && matches!(opt, Spring::Damaged)
                    {
                        // add over previous contiguous permutations
                        memo[pos][group][damaged] += memo[pos + 1][group][damaged + 1];
                    } else if matches!(arrangement[pos], Spring::Operational | Spring::Unknown)
                        && matches!(opt, Spring::Operational)
                        && damaged == 0
                    {
                        // add over the new group values
                        memo[pos][group][damaged] += memo[pos + 1][group][0];
                    } else if matches!(arrangement[pos], Spring::Operational | Spring::Unknown)
                        && matches!(opt, Spring::Operational)
                        && damaged_counts[group] == damaged
                    {
                        // add over permutations from previous group
                        memo[pos][group][damaged] += memo[pos + 1][group + 1][0];
                    }
                }
            }
        }
        if matches!(arrangement[pos], Spring::Operational | Spring::Unknown) {
            // propagate base case for trailing operational nodes (or unknowns we may need to prop from)
            memo[pos][max_groups][0] += memo[pos + 1][max_groups][0];
        }
    }
    // for item in memo.iter().rev() {
    //     println!("{:?}", item);
    // }

    memo[0][0][0]
}

fn part2(lines: &Vec<String>) -> String {
    let records = parse(lines);

    let futs = records
        .iter()
        .map(|(springs, counts)| {
            let mut newsprings = springs.clone();
            newsprings.push(Spring::Unknown);
            let mut newnewsprings = newsprings.repeat(5);
            newnewsprings.remove(newnewsprings.len() - 1);
            (newnewsprings, counts.repeat(5))
        })
        .map(|r| task::spawn(async move { process_record_fast(&r.clone()) }))
        .collect_vec();

    let sum: usize = block_on(async { join_all(futs).await }).iter().sum();

    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "???.### 1,1,3".to_string(),
            ".??..??...?##. 1,1,3".to_string(),
            "?#?#?#?#?#?#?#? 1,3,1,6".to_string(),
            "????.#...#... 4,1,1".to_string(),
            "????.######..#####. 1,6,5".to_string(),
            "?###???????? 3,2,1".to_string(),
        ]
    }
    fn sampledata_1() -> Vec<String> {
        vec!["???.### 1,1,3".to_string()]
    }
    fn sampledata_2() -> Vec<String> {
        vec![".??..??...?##. 1,1,3".to_string()]
    }
    fn sampledata_3() -> Vec<String> {
        vec!["?#?#?#?#?#?#?#? 1,3,1,6".to_string()]
    }
    fn sampledata_4() -> Vec<String> {
        vec!["????.#.#. 4,1,1".to_string()]
    }
    fn sampledata_5() -> Vec<String> {
        vec!["????.######..#####. 1,6,5".to_string()]
    }
    fn sampledata_6() -> Vec<String> {
        vec!["?###???????? 3,2,1".to_string()]
    }

    #[rstest]
    #[case(sampledata_1(), "0")]
    #[case(sampledata_2(), "4")]
    #[case(sampledata_3(), "1")]
    #[case(sampledata_4(), "0")]
    #[case(sampledata_5(), "4")]
    #[case(sampledata_6(), "10")]
    #[case(sampledata(), "21")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata_1(), "1")]
    #[case(sampledata_2(), "16384")]
    #[case(sampledata_3(), "1")]
    #[case(sampledata_4(), "16")]
    #[case(sampledata_5(), "2500")]
    #[case(sampledata_6(), "506250")]
    #[case(sampledata(), "525152")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}
