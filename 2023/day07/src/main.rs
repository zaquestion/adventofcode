use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum HandType {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn part1(lines: &Vec<String>) -> String {
    let mut hands = parse(lines, 11);

    hands.sort_by(|a, b| sort_hand(a, b));

    let sum = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i + 1))
        .sum::<usize>();

    // WRONG: 251382452
    format!("{}", sum)
}

fn part2(lines: &Vec<String>) -> String {
    let mut hands = parse(lines, 1);

    // let mut betterhands = hands
    //     .iter()
    //     .map(|(cards, _, bid)| {
    //         let mut hand_options: Vec<HandType> = cards
    //             .iter()
    //             .map(|&c| {
    //                 if c == 1 {
    //                     (2..14).filter(|&x| x != 11).collect_vec()
    //                 } else {
    //                     vec![c]
    //                 }
    //             })
    //             .multi_cartesian_product()
    //             .map(|combo| resolve_hand_type(&combo.into_iter().collect_vec()))
    //             .collect_vec();

    //         hand_options.sort_by(|a, b| sort_hand_by_type(a, b));
    //         let best_hand_type = hand_options.first().expect("elem").clone();
    //         (cards.clone(), best_hand_type, *bid)
    //     })
    //     .collect_vec();

    // betterhands.sort_by(|a, b| sort_hand(a, b));
    // Initally had this commented out code which produce the wrong answer before sorting out that
    // I could just match all of the joker cases

    hands.sort_by(|a, b| sort_hand(a, b));
    let sum = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i + 1))
        .sum::<usize>();

    // WRONG: 252024607
    format!("{}", sum)
}

fn sort_hand(a: &(Vec<usize>, HandType, usize), b: &(Vec<usize>, HandType, usize)) -> Ordering {
    if a.1 > b.1 {
        Ordering::Greater
    } else if a.1 == b.1 {
        a.0.iter()
            .interleave(b.0.iter())
            .tuples()
            .find(|(&aa, &bb)| aa != bb)
            .map_or(Ordering::Equal, |(&aa, &bb)| {
                if aa < bb {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
    } else {
        Ordering::Less
    }
}

fn sort_hand_by_type(a: &HandType, b: &HandType) -> Ordering {
    if a > b {
        Ordering::Greater
    } else if a == b {
        Ordering::Equal
    } else {
        Ordering::Less
    }
}

fn resolve_hand_type(cards: &Vec<usize>) -> HandType {
    let groups = cards
        .iter()
        .sorted()
        .group_by(|&c| c)
        .into_iter()
        .map(|(&k, grp)| (grp.count(), k))
        .sorted()
        .rev()
        .collect_vec();

    let num_jokers: usize = *groups
        .iter()
        .find(|(cnt, k)| *k == 1)
        .map(|(cnt, k)| cnt)
        .unwrap_or(&0);

    dbg!(&groups, &num_jokers);
    match groups.len() {
        1 => Some(HandType::FiveOfKind),
        2 => match groups.first().expect("must elem").0 {
            4 => match num_jokers {
                1 | 4 => Some(HandType::FiveOfKind),
                0 => Some(HandType::FourOfKind),
                _ => None,
            },
            3 => match num_jokers {
                2 | 3 => Some(HandType::FiveOfKind),
                1 => Some(HandType::FourOfKind),
                0 => Some(HandType::FullHouse),
                _ => None,
            },
            _ => None,
        },
        3 => match groups.first().expect("must elem").0 {
            3 => match num_jokers {
                1 | 3 => Some(HandType::FourOfKind),
                0 => Some(HandType::ThreeOfKind),
                _ => None,
            },
            2 => match num_jokers {
                2 => Some(HandType::FourOfKind),
                1 => Some(HandType::FullHouse),
                0 => Some(HandType::TwoPair),
                _ => None,
            },
            _ => None,
        },
        4 => match num_jokers {
            1 | 2 => Some(HandType::ThreeOfKind),
            0 => Some(HandType::OnePair),
            _ => None,
        },
        5 => match num_jokers {
            1 => Some(HandType::OnePair),
            0 => Some(HandType::HighCard),
            _ => None,
        },
        _ => None,
    }
    .expect("must resolve")
}

fn parse(lines: &Vec<String>, jvalue: usize) -> Vec<(Vec<usize>, HandType, usize)> {
    let mut card_values: HashMap<char, usize> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);
    card_values.insert('J', jvalue);
    lines
        .iter()
        .map(move |l| {
            l.split_whitespace()
                .tuples()
                .map(|(cards, bid)| {
                    let card_vec = cards
                        .chars()
                        .map(|c| card_values.get(&c).expect("must resolve"))
                        .map(|&u| u)
                        .collect_vec();
                    let typ = resolve_hand_type(&card_vec);
                    (card_vec, typ, bid.parse::<usize>().expect("must num"))
                })
                .exactly_one()
                .expect("must one")
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sampledata() -> Vec<String> {
        vec![
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ]
    }

    #[test]
    fn test_part1_sample() -> Result<(), String> {
        assert_eq!("6440", part1(&sampledata()));
        Ok(())
    }

    #[test]
    fn test_part2_sample() -> Result<(), String> {
        assert_eq!("5905", part2(&sampledata()));
        Ok(())
    }
}
