use colored::Colorize;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;

use itertools::Itertools;
use std::collections::HashSet;
use std::io;
use std::ops::{Add, Sub};

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Cell {
    Empty,
    FMirror,
    BMirror,
    VSplit,
    HSplit,
    Energized,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point(isize, isize);
// type Point = (isize, isize);

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point {
            0: self.0 - other.0,
            1: self.1 - other.1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Dir {
    UpWard,
    RightWard,
    LeftWard,
    DownWard,
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Cell>> {
    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| {
                    match c {
                        '.' => Some(Cell::Empty),
                        '/' => Some(Cell::FMirror),
                        '\\' => Some(Cell::BMirror),
                        '|' => Some(Cell::VSplit),
                        '-' => Some(Cell::HSplit),
                        _ => None,
                    }
                    .expect("must resolve")
                })
                .collect_vec()
        })
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> String {
    let board = parse(lines);

    format!("{}", simulate(&board, (Dir::RightWard, Point(0, 0))))
}

fn part2(lines: &Vec<String>) -> String {
    let board = parse(lines);

    let max: usize = (0..board.len())
        .flat_map(|y| {
            vec![
                (Dir::RightWard, Point(y as isize, 0)),
                (
                    Dir::LeftWard,
                    Point(y as isize, board[0].len() as isize - 1),
                ),
            ]
        })
        .chain((0..board[0].len()).flat_map(|x| {
            vec![
                (Dir::DownWard, Point(0, x as isize)),
                (Dir::UpWard, Point(board.len() as isize - 1, x as isize)),
            ]
        }))
        .collect_vec()
        .par_iter()
        .progress()
        .map(|start| simulate(&board, start.clone()))
        .max()
        .expect("must max");
    format!("{}", max)
}

fn simulate(board: &Vec<Vec<Cell>>, entry: (Dir, Point)) -> usize {
    let movedir = |dir: Dir| match dir {
        Dir::UpWard => Point(-1, 0),
        Dir::RightWard => Point(0, 1),
        Dir::LeftWard => Point(0, -1),
        Dir::DownWard => Point(1, 0),
    };

    let mut energized: HashSet<(Dir, Point)> = HashSet::new();
    energized.insert(entry.clone());
    let mut rays: Vec<(Dir, Point)> = vec![entry.clone()];
    loop {
        let old_len = energized.len();
        let newrays = rays
            .drain(0..)
            .map(|(d, cur)| {
                let Point(y, x) = cur;
                match board[y as usize][x as usize] {
                    Cell::Empty => vec![(d, &cur + &movedir(d))],
                    Cell::FMirror => match d {
                        Dir::UpWard => vec![(Dir::RightWard, &cur + &movedir(Dir::RightWard))],
                        Dir::DownWard => vec![(Dir::LeftWard, &cur + &movedir(Dir::LeftWard))],
                        Dir::LeftWard => vec![(Dir::DownWard, &cur + &movedir(Dir::DownWard))],
                        Dir::RightWard => vec![(Dir::UpWard, &cur + &movedir(Dir::UpWard))],
                    },
                    Cell::BMirror => match d {
                        Dir::UpWard => vec![(Dir::LeftWard, &cur + &movedir(Dir::LeftWard))],
                        Dir::DownWard => vec![(Dir::RightWard, &cur + &movedir(Dir::RightWard))],
                        Dir::LeftWard => vec![(Dir::UpWard, &cur + &movedir(Dir::UpWard))],
                        Dir::RightWard => vec![(Dir::DownWard, &cur + &movedir(Dir::DownWard))],
                    },
                    Cell::HSplit => match d {
                        Dir::LeftWard | Dir::RightWard => vec![(d, &cur + &movedir(d))],
                        Dir::UpWard | Dir::DownWard => {
                            vec![
                                (Dir::LeftWard, &cur + &movedir(Dir::LeftWard)),
                                (Dir::RightWard, &cur + &movedir(Dir::RightWard)),
                            ]
                        }
                    },
                    Cell::VSplit => match d {
                        Dir::UpWard | Dir::DownWard => vec![(d, &cur + &movedir(d))],
                        Dir::LeftWard | Dir::RightWard => {
                            vec![
                                (Dir::UpWard, &cur + &movedir(Dir::UpWard)),
                                (Dir::DownWard, &cur + &movedir(Dir::DownWard)),
                            ]
                        }
                    },
                    Cell::Energized => panic!("oh nooooooo"),
                }
            })
            .flatten()
            .filter(|(_, p)| {
                (0isize..board.len() as isize).contains(&p.0)
                    && (0isize..board[0].len() as isize).contains(&p.1)
            })
            .inspect(|(d, p)| {
                energized.insert((*d, p.clone()));
                // let points: HashSet<Point> =
                //     energized.iter().map(|(_, p)| p.clone()).unique().collect();
                // render(&board, &points);
            })
            .collect_vec();
        rays.extend(newrays);
        if rays.is_empty() {
            break;
        }
        if energized.len() == old_len {
            break;
        }
    }
    let points: HashSet<Point> = energized.iter().map(|(_, p)| p.clone()).unique().collect();
    // render(&board, &points);
    points.len()
}

fn render(b: &Vec<Vec<Cell>>, energized: &HashSet<Point>) {
    println!("\x1Bc");
    let mut colidx = (0..=2).into_iter().cycle();
    (0..b.len()).for_each(|y| {
        (0..b[0].len()).for_each(|x| {
            if energized.contains(&Point(y as isize, x as isize)) {
                match colidx.next().expect("must iter") {
                    0 => {
                        print!("{}", "#");
                    }
                    1 => {
                        print!("{}", "#".bright_yellow());
                    }
                    2 => {
                        print!("{}", "#".purple());
                    }
                    _ => panic!("eep!"),
                }
            } else {
                print!(".");
            }
        });
        println!("");
    });
    println!("");
    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            r".|...\....".to_string(),
            r"|.-.\.....".to_string(),
            r".....|-...".to_string(),
            r"........|.".to_string(),
            r"..........".to_string(),
            r".........\".to_string(),
            r"..../.\\..".to_string(),
            r".-.-/..|..".to_string(),
            r".|....-|.\".to_string(),
            r"..//.|....".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "46")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "51")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}
