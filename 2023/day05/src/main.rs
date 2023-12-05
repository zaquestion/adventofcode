use indicatif::ProgressIterator;
use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> String {
    let (seeds, mappers) = parse(lines);

    let min_loc = seeds
        .iter()
        .map(|s| mappers.iter().fold(*s, |acc, f| f(acc)))
        .min()
        .expect("must min");

    format!("{}", min_loc)
}

fn part2(lines: &Vec<String>) -> String {
    let (seed_spec, mappers) = parse(lines);

    let seeds = seed_spec
        .iter()
        .progress()
        .tuples()
        .flat_map(|(start, range)| (*start..*start + *range))
        .collect_vec();

    let min_loc = seeds
        .iter()
        .progress()
        .map(|s| mappers.iter().fold(*s, |acc, f| f(acc)))
        .min()
        .expect("must min");

    format!("{}", min_loc)
}

fn parse(lines: &Vec<String>) -> (Vec<usize>, Vec<impl Fn(usize) -> usize>) {
    // fn parse(lines: &Vec<String>) -> (Vec<usize>, Vec<Box<dyn Fn(usize) -> usize>>) {
    let mut iter = lines.split(|l| l.is_empty());

    let seeds = iter
        .next()
        .expect("must elem")
        .first()
        .expect("must line")
        .strip_prefix("seeds: ")
        .expect("must prefix")
        .split_whitespace()
        .map(|n| n.parse::<usize>().expect("must num"))
        .collect::<Vec<usize>>();

    let mappers = iter
        .map(|split| {
            let transforms = split
                .iter()
                .skip(1)
                .map(line_to_nums)
                .map(|l| {
                    move |v: usize| -> Option<usize> {
                        match v {
                            v if (l[1]..l[1] + l[2]).contains(&v) => Some(l[0] + (v - l[1])),
                            _ => None,
                        }
                    }
                })
                .collect::<Vec<_>>();
            move |v| transforms.iter().find_map(|f| f(v)).unwrap_or(v)
        })
        .collect::<Vec<_>>();

    (seeds, mappers)
}

fn line_to_nums(l: &String) -> Vec<usize> {
    l.split_whitespace()
        .map(|n| n.parse::<usize>().expect("must num"))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sampledata() -> Vec<String> {
        vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string(),
        ]
    }

    #[test]
    fn test_part1_sample() -> Result<(), String> {
        assert_eq!("35", part1(&sampledata()));
        Ok(())
    }

    #[test]
    fn test_part2_sample() -> Result<(), String> {
        assert_eq!("46", part2(&sampledata()));
        Ok(())
    }
}
