use itertools::Itertools;
use std::ops::Add;
use std::ops::Sub;
use std::{collections::HashMap, collections::HashSet, io};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn simpledata() -> Vec<String> {
        vec![
            "..........".to_string(),
            "...#......".to_string(),
            "#.........".to_string(),
            "....a.....".to_string(),
            "........a.".to_string(),
            ".....a....".to_string(),
            "..#.......".to_string(),
            "......#...".to_string(),
            "..........".to_string(),
            "..........".to_string(),
        ]
    }
    fn simpledata2() -> Vec<String> {
        vec![
            "T....#....".to_string(),
            "...T......".to_string(),
            ".T....#...".to_string(),
            ".........#".to_string(),
            "..#.......".to_string(),
            "..........".to_string(),
            "...#......".to_string(),
            "..........".to_string(),
            "....#.....".to_string(),
            "..........".to_string(),
        ]
    }
    fn sampledata() -> Vec<String> {
        vec![
            "......#....#".to_string(),
            "...#....0...".to_string(),
            "....#0....#.".to_string(),
            "..#....0....".to_string(),
            "....0....#..".to_string(),
            ".#....A.....".to_string(),
            "...#........".to_string(),
            "#......#....".to_string(),
            "........A...".to_string(),
            ".........A..".to_string(),
            "..........#.".to_string(),
            "..........#.".to_string(),
        ]
    }

    #[rstest]
    #[case(simpledata(), "4")]
    #[case(sampledata(), "14")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(simpledata2(), "9")]
    #[case(sampledata(), "34")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Empty,
    Sat(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn parse(lines: &Vec<String>) -> ((isize, isize), HashMap<char, HashSet<Point>>) {
    let mut out: HashMap<char, HashSet<Point>> = HashMap::new();
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' | '#' => None,
                _ => Some((
                    c,
                    Point {
                        x: x as isize,
                        y: y as isize,
                    },
                )),
            })
        })
        .for_each(|(c, p)| {
            if let Some(s) = out.get_mut(&c) {
                s.insert(p);
            } else {
                out.insert(c, HashSet::from([p]));
            }
        });
    ((lines[0].len() as isize, lines.len() as isize), out)
}

fn part1(lines: &Vec<String>) -> String {
    let ((maxx, maxy), sats) = parse(lines);
    let antipoints = sats
        .iter()
        .flat_map(|(_, locs)| {
            locs.iter().tuple_combinations().flat_map(|(a, b)| {
                let d = a - b;
                [a + &d, &(a - &d) - &d].into_iter()
            })
        })
        .filter(|p| p.x >= 0 && p.x < maxx && p.y >= 0 && p.y < maxy)
        .unique()
        .count();

    format!("{}", antipoints)
}

fn part2(lines: &Vec<String>) -> String {
    let ((maxx, maxy), sats) = parse(lines);
    let antipoints = sats
        .iter()
        .flat_map(|(_, locs)| {
            locs.iter().tuple_combinations().flat_map(|(a, b)| {
                let mut newpoints: Vec<Point> = Vec::new();
                let d = a - b;
                let mut p = *a;
                while p.x >= 0 && p.x < maxx && p.y >= 0 && p.y < maxy {
                    let pp = p.clone();
                    newpoints.push(pp);
                    p = &p + &d;
                }
                p = *a;
                while p.x >= 0 && p.x < maxx && p.y >= 0 && p.y < maxy {
                    let pp = p.clone();
                    newpoints.push(pp);
                    p = &p - &d;
                }
                newpoints
            })
        })
        // .filter(|p| p.x >= 0 && p.x < maxx && p.y >= 0 && p.y < maxy)
        .unique()
        .count();

    format!("{}", antipoints)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    // WRONG: 367
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
