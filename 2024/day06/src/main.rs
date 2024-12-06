use itertools::Itertools;
use std::collections::HashSet;
use std::io;
use std::ops::{Add, Index};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "41")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "6")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Guard,
    Obstruction,
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

const DIRS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

fn parse(lines: &Vec<String>) -> (Point, Vec<Vec<Cell>>) {
    let mut starting: Point = Point { x: -1, y: -1 };
    let board = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Cell::Empty,
                    '^' => {
                        starting = Point {
                            x: x as isize,
                            y: y as isize,
                        };
                        Cell::Guard
                    }
                    '#' => Cell::Obstruction,
                    _ => panic!("must resolve"),
                })
                .collect_vec()
        })
        .collect_vec();
    (starting, board)
}

fn part1(lines: &Vec<String>) -> String {
    let (mut pos, board) = parse(lines);
    let mut points: HashSet<Point> = HashSet::from([pos]);
    let mut dir_idx = 3;
    let mut dir = DIRS[dir_idx];
    loop {
        let next = &pos + &dir;
        if next.x < 0
            || next.x > board[0].len() as isize - 1
            || next.y < 0
            || next.y > board.len() as isize - 1
        {
            break;
        }
        if board[next.y as usize][next.x as usize] == Cell::Obstruction {
            dir_idx = (dir_idx + 1) % 4;
            dir = DIRS[dir_idx];
            continue;
        }
        points.insert(next);
        pos = next;
    }

    format!("{}", points.len())
}

fn part2(lines: &Vec<String>) -> String {
    let (starting_pos, starting_board) = parse(lines);
    let mut options: HashSet<Point> = HashSet::new();

    for (oy, ox) in (0..starting_board.len()).cartesian_product(0..starting_board[0].len()) {
        if starting_board[oy][ox] != Cell::Empty {
            continue;
        }
        if starting_pos
            == (Point {
                x: ox as isize,
                y: oy as isize,
            })
        {
            continue;
        }
        let mut board = starting_board.clone();
        let mut pos = starting_pos.clone();
        board[oy][ox] = Cell::Obstruction;
        let mut dir_idx = 3;
        let mut dir = DIRS[dir_idx];
        let mut points: HashSet<(Point, Point)> = HashSet::from([(DIRS[3], starting_pos.clone())]);
        loop {
            let next = &pos + &dir;
            if next.x < 0
                || next.x > board[0].len() as isize - 1
                || next.y < 0
                || next.y > board.len() as isize - 1
            {
                break;
            }
            if board[next.y as usize][next.x as usize] == Cell::Obstruction {
                dir_idx = (dir_idx + 1) % 4;
                dir = DIRS[dir_idx];
                continue;
            }

            pos = next;
            if points.contains(&(dir, pos)) {
                options.insert(Point {
                    x: ox as isize,
                    y: oy as isize,
                });
                break;
            }
            points.insert((dir, pos));
        }
    }

    format!("{}", options.len())
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
