use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    // WRONG: 4040
    // WRONG: 3684
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

struct Game {
    id: usize,
    sets: Vec<(usize, usize, usize)>,
}

fn part1(lines: &Vec<String>) -> String {
    let sum = lines
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

            let sets = parts[1]
                .split(";")
                .map(|s| {
                    s.split_whitespace()
                        .tuples()
                        .filter_map(|(num, color)| {
                            let n = num.parse::<usize>().expect("must num");
                            let c = color.trim_end_matches(",");
                            match c {
                                "red" => Some((n, 0, 0)),
                                "green" => Some((0, n, 0)),
                                "blue" => Some((0, 0, n)),
                                _ => None,
                            }
                        })
                        .fold((0, 0, 0), |(r, g, b), (sr, gr, br)| {
                            (r + sr, g + gr, b + br)
                        })
                })
                .collect::<Vec<_>>();

            Game { id: id, sets: sets }
        })
        .filter_map(|g| {
            if g.sets.iter().all(|s| s.0 <= 12 && s.1 <= 13 && s.2 <= 14) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum::<usize>();

    return format!("{}", sum);
}

fn part2(lines: &Vec<String>) -> String {
    let sum = lines
        .iter()
        .map(|l| {
            let parts = l.split(":").collect::<Vec<_>>();
            parts[1]
                .split(";")
                .map(|s| {
                    s.split_whitespace()
                        .tuples()
                        .filter_map(|(num, color)| {
                            let n = num.parse::<usize>().expect("must num");
                            let c = color.trim_end_matches(",");
                            match c {
                                "red" => Some((n, 0, 0)),
                                "green" => Some((0, n, 0)),
                                "blue" => Some((0, 0, n)),
                                _ => None,
                            }
                        })
                        .fold((0, 0, 0), |(r, g, b), (sr, gr, br)| {
                            (r + sr, g + gr, b + br)
                        })
                })
                .fold(
                    (usize::MIN, usize::MIN, usize::MIN),
                    |(r, g, b), (sr, gr, br)| {
                        (
                            if r > sr { r } else { sr },
                            if g > gr { g } else { gr },
                            if b > br { b } else { br },
                        )
                    },
                )
        })
        .map(|(r, g, b)| r * g * b)
        .sum::<usize>();

    return format!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part2_sample() -> Result<(), String> {
        let input: Vec<String> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!("2286", part2(&input));
        Ok(())
    }

    #[test]
    fn test_part1_sample() -> Result<(), String> {
        let input: Vec<String> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!("8", part1(&input));
        Ok(())
    }

    #[test]
    fn test_part1_badgreen() -> Result<(), String> {
        let input: Vec<String> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 20 green
Game 2: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!("2", part1(&input));
        Ok(())
    }

    #[test]
    fn test_part1_badblue() -> Result<(), String> {
        let input: Vec<String> = "Game 1: 3 blue, 4 red; 1 red, 2 green, 20 blue; 2 green
Game 2: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!("2", part1(&input));
        Ok(())
    }

    #[test]
    fn test_part1_badred() -> Result<(), String> {
        let input: Vec<String> = "Game 1: 3 blue, 20 red; 1 red, 2 green, 2 blue; 2 green
Game 2: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!("2", part1(&input));
        Ok(())
    }

    #[test]
    fn test_part1_input5() -> Result<(), String> {
        let input: Vec<String> = "Game 1: 19 blue, 12 red; 19 blue, 2 green, 1 red; 13 red, 11 blue
Game 2: 1 green, 1 blue, 1 red; 11 red, 3 blue; 1 blue, 18 red; 9 red, 1 green; 2 blue, 11 red, 1 green; 1 green, 2 blue, 10 red
Game 3: 3 blue, 2 red, 6 green; 4 blue, 6 green, 1 red; 11 green, 12 blue; 2 red, 6 green, 4 blue; 4 green
Game 4: 10 red, 5 green, 5 blue; 3 red, 3 blue, 6 green; 2 blue, 9 red, 6 green; 8 green, 10 red, 4 blue; 9 red, 2 green, 3 blue; 1 blue, 5 red, 15 green
Game 5: 11 green, 7 blue; 5 green, 5 red, 1 blue; 1 green, 1 red, 4 blue; 1 red, 1 blue, 4 green; 4 blue, 1 red, 10 green; 5 red, 6 green"
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!("8", part1(&input));
        Ok(())
    }
}
