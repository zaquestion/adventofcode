use itertools::Itertools;
use std::collections::HashSet;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Clone)]
struct ScratchCard {
    id: usize,
    mine: HashSet<usize>,
    winning: HashSet<usize>,
    copies: usize,
}

fn part1(lines: &Vec<String>) -> String {
    let cards = parse(lines);

    let points: usize = cards
        .iter()
        .map(|c| {
            let winners = c.mine.intersection(&c.winning).count();
            if winners == 0 {
                0
            } else {
                (2 as usize).pow(winners as u32 - 1)
            }
        })
        .sum();

    format!("{}", points)
}

fn part2(lines: &Vec<String>) -> String {
    let mut cards = parse(lines);

    let total_cards: usize = (0..cards.len())
        .map(|i| {
            let c = &mut cards[i];
            let copies = c.copies.clone();
            let winners = c.mine.intersection(&c.winning).count();
            (c.id..c.id + winners).for_each(|i| cards[i].copies += copies);
            copies
        })
        .sum();

    format!("{}", total_cards)
}

fn parse(lines: &Vec<String>) -> Vec<ScratchCard> {
    lines
        .iter()
        .map(|l| {
            let parts = l.split(":").collect::<Vec<_>>();
            let id = parts[0]
                .split_whitespace()
                .skip(1)
                .take(1)
                .exactly_one()
                .expect("must one")
                .parse::<usize>()
                .expect("must num");

            let (winning, mine): (HashSet<usize>, HashSet<usize>) = parts[1]
                .split("|")
                .map(|nums| {
                    nums.to_string()
                        .split_whitespace()
                        .map(|nstr| nstr.parse::<usize>().expect("must num"))
                        .collect::<HashSet<_>>()
                })
                .collect_tuple()
                .expect("must tuple");

            ScratchCard {
                id: id,
                winning: winning,
                mine: mine,
                copies: 1,
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sampledata() -> Vec<String> {
        vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ]
    }

    #[test]
    fn test_part1_sample() -> Result<(), String> {
        assert_eq!("13", part1(&sampledata()));
        Ok(())
    }

    #[test]
    fn test_part2_sample() -> Result<(), String> {
        let input: Vec<String> = vec![];
        assert_eq!("30", part2(&sampledata()));
        Ok(())
    }
}
