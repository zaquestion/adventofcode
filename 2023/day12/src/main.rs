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

fn process_record_fast(record: &(Vec<Spring>, Vec<usize>)) -> usize {
    // (position, group_idx, contiguous damaged)

    // really helpful videos for helping me grok DynProg
    // https://www.youtube.com/watch?v=Hdr64lKQ3e4

    // damaged == group[idx]
    // . | # = 1 (no ?)

    // helix: mips\|<ret>&
    /*
        // .??..??...?##. 1,1,3

        | seq  | pos | group | damaged | -> |
        |  .   |  0  |   0   |    0    | 1  |

        |  .?  |  _  |   _   |    _    | _  |
        |  ..  |  1  |   0   |    0    | 1  |
        |  .#  |  1  |   0   |    1    | 1  |

        |  ..?  |  _  |   _   |    _    | _  |
        |  ...  |  2  |   0   |    0    | 1  |
        |  ..#  |  2  |   0   |    1    | 1  |
        |  .#?  |  _  |   _   |    _    | _  |
        |  .#.  |  2  |   1   |    0    | 1  |
        |  .##  |  2  |   0   |    2    | 1  |

        |  ..#..?  |  _  |   _   |    _    | _  |
        |  ..#...  |  5  |   1   |    0    | 1  |
        |  ..#..#  |  5  |   1   |    1    | 2  |

        |  .#...?  |  _  |   _   |    _    | _  |
        |  .#....  |  5  |   1   |    0    | 1  |
        |  .#...#  |  5  |   1   |    1    | 2  |


        | k    | g |  -> |
        | .??. | 1 | -> 2

    */

    // vec![".??..??...?##. 1,1,3".to_string()]
    let mut memo = HashMap::<(usize, usize, usize), usize>::new();
    let (arrangement, damaged_counts) = record;
    let max_groups = damaged_counts.len();
    memo.insert((0, 0, 0), 1); // initial state to prop from
                               // memo.insert((0, 1, 1), 1); // initial state to prop from

    let mut test_memo = HashMap::<(usize, usize), usize>::new();
    test_memo.insert((0, 0), 1); // initial state to prop from
    for pos in 1..arrangement.len() {
        test_memo.insert((pos, 0), 0);
        for (i, _opt) in &mut [Spring::Operational, Spring::Damaged].iter().enumerate() {
            let prev = test_memo.get(&(pos - 1, i)).unwrap_or(&0).clone();
            dbg!(prev);
            *test_memo.get_mut(&(pos, 0)).expect("must exist") += prev;
        }
        test_memo.iter().for_each(|(k, v)| {
            println!("{:?} - > {}", k, v);
        });
        println!();
        if pos == 3 {
            break;
        }
    }
    /*
    for pos in 1..arrangement.len() {
        for group in 1..damaged_counts.len() {
            for damaged in 1..damaged_counts[group] {
                if matches!(arrangement[pos], Spring::Damaged | Spring::Unknown) {
                    let prev = memo
                        .get(&(pos - 1, group - 1, damaged - 1))
                        .unwrap_or(&0)
                        .clone();
                    *memo.get_mut(&(pos, group, damaged)).unwrap_or(&mut 0) += prev;
                }
            }
            // if spring != Spring::Unknown {
            //     //memo.insert((pos, dc, )
            //     continue;
            // }
            // for opt in &[Spring::Operational, Spring::Damaged] {
            // }
            if matches!(arrangement[pos], Spring::Operational | Spring::Unknown) {
                // memo[pos][max_groups][0] += memo[pos - 1][max_groups][0];
                let prev = memo.get(&(pos - 1, 0, 0)).unwrap_or(&0).clone();
                *memo.get_mut(&(pos, 0, 0)).unwrap_or(&mut 0) += prev;
            }
        }
        memo.iter().for_each(|(k, v)| {
            println!("{:?} - > {}", k, v);
        });
        println!();
        if pos == 2 {
            break;
        }
    }
    */

    0
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
        .map(|r| task::spawn(async move { process_record(&r.clone()) }))
        .collect_vec();

    let sum: usize = block_on(async { join_all(futs).await }).iter().sum();
    // .inspect(|s| {
    //     dbg!(s);
    // })
    // .sum();

    format!("{}", sum)
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
        vec!["????.#...#... 4,1,1".to_string()]
    }
    fn sampledata_5() -> Vec<String> {
        vec!["????.######..#####. 1,6,5".to_string()]
    }
    fn sampledata_6() -> Vec<String> {
        vec!["?###???????? 3,2,1".to_string()]
    }

    #[rstest]
    // #[case(sampledata_1(), "1")]
    #[case(sampledata_2(), "4")]
    // #[case(sampledata_3(), "1")]
    // #[case(sampledata_4(), "1")]
    // #[case(sampledata_5(), "4")]
    // #[case(sampledata_6(), "10")]
    // #[case(sampledata(), "21")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    // #[rstest]
    // #[case(sampledata_1(), "1")]
    // #[case(sampledata_2(), "16384")]
    // #[case(sampledata_3(), "1")]
    // #[case(sampledata_4(), "16")]
    // #[case(sampledata_5(), "2500")]
    // #[case(sampledata_6(), "506250")]
    // #[case(sampledata(), "525152")]
    // fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
    //     assert_eq!(expected, part2(&input));
    // }
}

// #[cfg(test)]
// mod tests_subset {
//     extern crate test;

//     use test::Bencher;

//     use super::*;
//     use rstest::rstest;
//     // DP practice problem to get my comprehension up
//     // https://www.techiedelight.com/subset-sum-problem/
//     #[rstest]
//     // #[case(vec![7, 3, 2, 5, 8], 14, true)]
//     #[case(vec![1, 3, 2, 4], 5, true)]
//     // #[case(vec![
//     //     104, 102, 202, 204, 305, 403, 505, 108, 306, 405,
//     //     101, 206, 303, 409, 501, 107, 302, 402, 508, 110,
//     //     205, 307, 404
//     //             ], 1312, true)]
//     fn test_subset_sum_memo(
//         #[case] nums: Vec<usize>,
//         #[case] target: usize,
//         #[case] expected: bool,
//     ) {
//         assert_eq!(expected, subset_sum_memo_gpt(nums, target));
//     }
//     #[bench]
//     fn bench_subset_sum_memo(b: &mut Bencher) {
//         b.iter(|| {
//             subset_sum_memo_gpt(
//                 vec![
//                     104, 102, 202, 204, 305, 403, 505, 108, 306, 405, 101, 206, 303, 409, 501, 107,
//                     302, 402, 508, 110, 205, 307, 404,
//                 ],
//                 2500,
//             )
//         })
//     }

//     // fn subset_sum_memo(nums: Vec<usize>, target: usize) -> bool {
//     //     let mut memo = HashMap::<(usize, usize), usize>::new();
//     //     memo.insert((0, 0), 1);
//     //     // (position, length)
//     //     //

//     //     for cur_sum in 0..target {
//     //         memo.get(
//     //             for (idx, n) in nums.iter().enumerate().skip(1) {
//     //             // if cur_sum == 0 {
//     //             //     memo.insert((idx, 0), *n);
//     //             //     continue;
//     //             // }
//     //             // if cur_sum == 2 {
//     //             //     break;
//     //             // }
//     //             // let new = n + memo.get(&(*n, cur_sum - 1)).expect("must exist");
//     //             // if new == target {
//     //             //     return true;
//     //             // }
//     //             // memo.insert((idx, cur_sum), new);
//     //             // println!("{}", new);
//     //             // memo.iter()
//     //             //     .for_each(|(k, v)| println!("({:?}) -> {}", k, v))
//     //         }
//     //         memo.iter()
//     //             .filter(|(&k, &v)| k.0 == cur_sum)
//     //             .sorted_by(|(_, a), (_, b)| a.cmp(b))
//     //             .for_each(|(k, v)| {
//     //                 println!("{:?} -> {}", k, v);
//     //                 // println!(
//     //                 //     "{:?} ({} - {} = {}) -> {}",
//     //                 //     k,
//     //                 //     k.1,
//     //                 //     nums[i],
//     //                 //     k.1 - nums[i],
//     //                 //     v
//     //                 // )
//     //             });
//     //         println!();
//     //     }

//     //     // memo.get((0, 2))
//     //     false
//     // }

//     fn subset_sum_memo_gpt(nums: Vec<usize>, target: usize) -> bool {
//         let n = nums.len();
//         let mut memo: HashMap<(usize, usize), bool> = HashMap::new();

//         for i in 0..=n {
//             if i == 3 {
//                 break;
//             }
//             // #[case(vec![1, 3, 2, 4], 5, true)]
//             for current_sum in 0..=target {
//                 if current_sum == 0 && i == 0 {
//                     memo.insert((i, current_sum), true);
//                 } else if i == 0 {
//                     memo.insert((i, current_sum), false);
//                 } else {
//                     let include = if current_sum >= nums[i - 1] {
//                         *memo
//                             .get(&(i - 1, current_sum - nums[i - 1]))
//                             .unwrap_or(&false)
//                     } else {
//                         false
//                     };
//                     let exclude = *memo.get(&(i - 1, current_sum)).unwrap_or(&false);
//                     memo.insert((i, current_sum), include || exclude);
//                 }
//             }
//             memo.iter()
//                 .filter(|(&k, &v)| k.0 == i && v)
//                 .sorted_by(|(_, a), (_, b)| a.cmp(b))
//                 .for_each(|(k, v)| {
//                     println!("{:?} -> {}, num: {}", k, v, nums[i]);
//                     // println!(
//                     //     "{:?} ({} - {} = {}) -> {}",
//                     //     k,
//                     //     k.1,
//                     //     nums[i],
//                     //     k.1 - nums[i],
//                     //     v
//                     // )
//                 });
//             println!();
//         }

//         *memo.get(&(n, target)).unwrap_or(&false)
//     }

//     #[rstest]
//     #[case(vec![7, 3, 2, 5, 8], 14, true)]
//     #[case(vec![
//         104, 102, 202, 204, 305, 403, 505, 108, 306, 405,
//         101, 206, 303, 409, 501, 107, 302, 402, 508, 110,
//         205, 307, 404
//                 ], 1312, true)]
//     fn test_subset_sum_brute(
//         #[case] nums: Vec<usize>,
//         #[case] target: usize,
//         #[case] expected: bool,
//     ) {
//         assert_eq!(expected, subset_sum_brute(nums, target));
//     }

//     #[bench]
//     fn bench_subset_sum_brute(b: &mut Bencher) {
//         b.iter(|| {
//             subset_sum_brute(
//                 vec![
//                     104, 102, 202, 204, 305, 403, 505, 108, 306, 405, 101, 206, 303, 409, 501, 107,
//                     302, 402, 508, 110, 205, 307, 404,
//                 ],
//                 2500,
//             )
//         })
//     }

//     fn subset_sum_brute(nums: Vec<usize>, target: usize) -> bool {
//         let mut subsets = vec![vec![]];

//         for num in nums {
//             let current_size = subsets.len();
//             for j in 0..current_size {
//                 let mut new_subset = subsets[j].clone();
//                 new_subset.push(num);
//                 subsets.push(new_subset);
//             }
//         }

//         subsets
//             .into_iter()
//             .any(|subset| subset.iter().sum::<usize>() == target)
//     }
// }
