use itertools::Itertools;
use std::collections::HashMap;
use std::io;
use std::ops::Add;
use std::ops::Sub;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    // WRONG: 104659
    println!("part 2: {:?}", part2(&lines, 1000000000));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Rounded,
    Cube,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point(isize, isize);

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

fn parse(lines: &Vec<String>) -> Vec<Vec<Cell>> {
    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| {
                    match c {
                        '.' => Some(Cell::Empty),
                        'O' => Some(Cell::Rounded),
                        '#' => Some(Cell::Cube),
                        _ => None,
                    }
                    .expect("must resolve")
                })
                .collect_vec()
        })
        .collect_vec()
}

fn render(b: &Vec<Vec<Cell>>) {
    b.iter().for_each(|l| {
        l.iter().for_each(|c| {
            print!(
                "{}",
                match c {
                    Cell::Empty => ".",
                    Cell::Rounded => "O",
                    Cell::Cube => "#",
                }
            );
        });
        println!("");
    });
    println!("");
}

fn part1(lines: &Vec<String>) -> String {
    let board = parse(lines);

    let sum: usize = tilt(&board, Direction::North)
        .iter()
        .rev()
        .enumerate()
        .rev()
        .fold(0, |acc, (y, l)| {
            acc + (l.iter().filter(|c| *c == &Cell::Rounded).count() * (y + 1))
        });

    format!("{}", sum)
}

fn part2(lines: &Vec<String>, cycles: usize) -> String {
    let mut board = parse(lines);

    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut boards: Vec<(usize, Vec<Vec<Cell>>)> = Vec::new();
    // render(&board);
    for i in 0..cycles {
        board = tilt(&board, Direction::North);
        board = tilt(&board, Direction::West);
        board = tilt(&board, Direction::South);
        board = tilt(&board, Direction::East);
        let k = key(&board);
        if cache.contains_key(&k) {
            let delta = i - cache.get(&k).expect("must key");
            println!("repeat: {}, delta: {}", i, delta);
            if boards.len() == delta {
                break;
            }
            boards.push((i, board.clone()));
        }
        cache.insert(k, i);
        // render(&board)
    }

    let finalboard = if boards.len() > 0 {
        let (lastidx, _) = boards[0];
        let (_, finalboard) = &boards[(cycles - lastidx - 1) % boards.len()];
        finalboard
    } else {
        &board
    };

    let sum: usize = finalboard
        .iter()
        .rev()
        .enumerate()
        .rev()
        .fold(0, |acc, (y, l)| {
            acc + (l.iter().filter(|c| *c == &Cell::Rounded).count() * (y + 1))
        });

    format!("{}", sum)
}

fn key(b: &Vec<Vec<Cell>>) -> String {
    b.iter()
        .flat_map(|l| l.iter())
        .map(|c| match c {
            Cell::Empty => ".",
            Cell::Rounded => "O",
            Cell::Cube => "#",
        })
        .collect::<String>()
}
fn tilt(b: &Vec<Vec<Cell>>, dir: Direction) -> Vec<Vec<Cell>> {
    let (max_y, max_x) = (b.len() as isize - 1, b[0].len() as isize - 1);

    let movedir = match dir {
        Direction::North => Point(-1, 0),
        Direction::East => Point(0, 1),
        Direction::West => Point(0, -1),
        Direction::South => Point(1, 0),
    };

    let mut prev = b.clone();
    loop {
        let mut next = prev.clone();

        let row_iter = if dir == Direction::South {
            (0..b.len()).rev().collect_vec()
        } else {
            (0..b.len()).collect_vec()
        };
        for y in row_iter {
            let col_iter = if dir == Direction::West {
                (0..b.first().expect("must first").len())
                    .rev()
                    .collect_vec()
            } else {
                (0..b.first().expect("must first").len()).collect_vec()
            };
            for x in col_iter {
                let Point(new_y, new_x) = &Point(y as isize, x as isize) + &movedir;
                if new_x < 0 || new_x > max_x || new_y < 0 || new_y > max_y {
                    continue;
                }
                let cur = prev[y][x];
                let new = prev[new_y as usize][new_x as usize];
                if cur == Cell::Rounded && new == Cell::Empty {
                    next[new_y as usize][new_x as usize] = cur;
                    next[y as usize][x as usize] = new;
                }
            }
        }

        if next
            .iter()
            .interleave(prev.iter())
            .tuples()
            .all(|(l, r)| l == r)
        {
            break;
        }
        prev = next.to_vec();
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "O....#....".to_string(),
            "O.OO#....#".to_string(),
            ".....##...".to_string(),
            "OO.#O....O".to_string(),
            ".O.....O#.".to_string(),
            "O.#..O.#.#".to_string(),
            "..O..#O..O".to_string(),
            ".......O..".to_string(),
            "#....###..".to_string(),
            "#OO..#....".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "136")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), 1, "87")]
    #[case(sampledata(), 1000000000, "64")]
    fn test_part2_sample(
        #[case] input: Vec<String>,
        #[case] cycle: usize,
        #[case] expected: String,
    ) {
        assert_eq!(expected, part2(&input, cycle));
    }
}
