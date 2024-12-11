use itertools::Itertools;
use std::io;
use std::ops::Add;
use std::ops::Sub;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn simpledata() -> Vec<String> {
        vec![
            "0123".to_string(),
            "1234".to_string(),
            "8765".to_string(),
            "9876".to_string(),
        ]
    }

    fn simpledata2() -> Vec<String> {
        vec![
            "8880888".to_string(),
            "8881888".to_string(),
            "8882888".to_string(),
            "6543456".to_string(),
            "7888887".to_string(),
            "8888888".to_string(),
            "9888889".to_string(),
        ]
    }
    fn simpledata3() -> Vec<String> {
        // 10..9..
        // 2...8..
        // 3...7..
        // 4567654
        // ...8..3
        // ...9..2
        // .....01
        vec![
            "1088988".to_string(),
            "2888888".to_string(),
            "3888788".to_string(),
            "4567654".to_string(),
            "8888883".to_string(),
            "8889882".to_string(),
            "8888801".to_string(),
        ]
    }

    fn sampledata() -> Vec<String> {
        vec![
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ]
    }

    #[rstest]
    #[case(simpledata(), "1")]
    #[case(simpledata2(), "2")]
    #[case(simpledata3(), "3")]
    #[case(sampledata(), "36")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "81")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
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

const DIRS: [Point; 4] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: 0, y: -1 },
];

fn parse(lines: &Vec<String>) -> (Vec<Point>, Vec<Point>, Vec<Vec<usize>>) {
    (
        lines
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().map(move |(x, c)| {
                    (
                        Point {
                            x: x as isize,
                            y: y as isize,
                        },
                        c.to_string().parse::<usize>().expect("must num"),
                    )
                })
            })
            .filter_map(|(p, num)| if num == 0 { Some(p) } else { None })
            .collect_vec(),
        lines
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().map(move |(x, c)| {
                    (
                        Point {
                            x: x as isize,
                            y: y as isize,
                        },
                        c.to_string().parse::<usize>().expect("must num"),
                    )
                })
            })
            .filter_map(|(p, num)| if num == 9 { Some(p) } else { None })
            .collect_vec(),
        lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_string().parse::<usize>().expect("must num"))
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    height: usize,
    path: Vec<Point>,
}

impl State {
    fn new(p: Point) -> State {
        State {
            height: 0,
            path: vec![p],
        }
    }
}

fn dist(s: &Point, b: &Point) -> isize {
    (s.x.abs_diff(b.x) + s.y.abs_diff(b.y)) as isize
}

fn climb(board: &Vec<Vec<usize>>, start: Point, end: Point, all: bool) -> usize {
    let (maxx, maxy) = (board[0].len() as isize - 1, board.len() as isize - 1);
    let initial = State::new(start.clone());
    let mut next: Vec<State> = vec![initial.clone()];
    let mut count = 0;

    while next.len() != 0 {
        let mut nextnext: Vec<State> = Vec::new();
        for state in next.iter() {
            let pos = state.path.last().expect("must last");
            for d in DIRS {
                let nextpos = pos + &d;
                if nextpos.x < 0 || nextpos.x > maxx || nextpos.y < 0 || nextpos.y > maxy {
                    continue;
                }
                if board[nextpos.y as usize][nextpos.x as usize] != state.height + 1 {
                    continue;
                }
                if state.height + 1 == 9 && nextpos == end {
                    count += 1;
                    if !all {
                        return count;
                    }
                    continue;
                }
                let mut new = state.clone();
                new.height = state.height + 1;
                new.path.push(nextpos);
                nextnext.push(new);
            }
        }
        next = nextnext;
    }
    return count;
}

fn part1(lines: &Vec<String>) -> String {
    let (starts, ends, board) = parse(lines);

    let score = starts
        .iter()
        .cartesian_product(ends.iter())
        .filter(|(&s, &e)| climb(&board, s, e, false) == 1)
        .count();

    format!("{}", score)
}

fn part2(lines: &Vec<String>) -> String {
    let (starts, ends, board) = parse(lines);

    let score: usize = starts
        .iter()
        .cartesian_product(ends.iter())
        .map(|(&s, &e)| climb(&board, s, e, true))
        .sum();

    format!("{}", score)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
