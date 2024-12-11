use itertools::Itertools;
use std::{collections::HashMap, io};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec!["125 17".to_string()]
    }

    #[rstest]
    #[case(sampledata(), 1, "3")]
    #[case(sampledata(), 2, "4")]
    #[case(sampledata(), 6, "22")]
    #[case(sampledata(), 25, "55312")]
    fn test_blink(#[case] input: Vec<String>, #[case] rounds: usize, #[case] expected: String) {
        assert_eq!(expected, blink(&input, rounds));
    }
}

/*
If the stone is engraved with the number 0, it is replaced by a stone engraved
with the number 1.

If the stone is engraved with a number that has an even number of digits, it is
replaced by two stones. The left half of the digits are engraved on the new left
stone, and the right half of the digits are engraved on the new right stone.
(The new numbers don't keep extra leading zeroes: 1000 would become stones 10
and 0.)

If none of the other rules apply, the stone is replaced by a new stone; the old
stone's number multiplied by 2024 is engraved on the new stone.
*/
fn strip_leading_zeros(s: &str) -> String {
    let stripped = s.trim_start_matches('0');
    if stripped.is_empty() {
        "0".to_string() // Return "0" if the string is empty after stripping
    } else {
        stripped.to_string() // Return the stripped string
    }
}

fn blink(lines: &Vec<String>, rounds: usize) -> String {
    let mut stones: HashMap<String, usize> = lines
        .first()
        .expect("must first")
        .split_whitespace()
        .map(|s| (s.to_string(), 1))
        .collect();

    for _ in 0..rounds {
        let mut new_stones: HashMap<String, usize> = HashMap::default();
        stones.iter().for_each(|(nstr, n)| {
            let res = {
                if nstr == "0" {
                    vec![("1".to_string(), n)]
                } else if nstr.len() % 2 == 0 {
                    let (l, r) = nstr.split_at(nstr.len() / 2);
                    vec![(strip_leading_zeros(l), n), (strip_leading_zeros(r), n)]
                } else {
                    vec![(
                        (nstr.parse::<usize>().expect("must num") * 2024).to_string(),
                        n,
                    )]
                }
            };
            res.iter().for_each(|(nstr, &n)| {
                new_stones
                    .entry(nstr.clone())
                    .and_modify(|e| *e += n)
                    .or_insert(n);
            });
        });

        stones = new_stones;
    }
    format!("{}", stones.values().sum::<usize>())
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", blink(&lines, 25));
    println!("part 2: {:?}", blink(&lines, 75));
}
