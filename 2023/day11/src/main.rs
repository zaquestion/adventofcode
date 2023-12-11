use itertools::Itertools;
use std::{collections::HashSet, io};

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines, 1000000));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Galaxy,
}

fn part1(lines: &Vec<String>) -> String {
    let board = parse(lines);
    let (max_y, max_x) = (board.len(), board[0].len());
    // render(&board);

    let galaxies = (0..max_y)
        .cartesian_product(0..max_x)
        .filter_map(|(y, x)| {
            if board[y][x] == Cell::Galaxy {
                Some((y, x))
            } else {
                None
            }
        })
        .collect_vec();

    let sum: usize = galaxies
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| Some(distance(a, b)))
        .sum();

    format!("{}", sum)
}

fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as usize
}

fn part2(lines: &Vec<String>, factor: usize) -> String {
    let (board, (rows, cols)) = parse2(lines);
    let (max_y, max_x) = (board.len(), board[0].len());
    // render(&board);

    let galaxies = (0..max_y)
        .cartesian_product(0..max_x)
        .filter_map(|(y, x)| {
            if board[y][x] == Cell::Galaxy {
                Some((y, x))
            } else {
                None
            }
        })
        .collect_vec();

    let sum: usize = galaxies
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            let (miny, maxy) = if a.0 > b.0 { (b.0, a.0) } else { (a.0, b.0) };
            let (minx, maxx) = if a.1 > b.1 { (b.1, a.1) } else { (a.1, b.1) };
            let expanded_y = (miny..maxy).filter(|c| rows.contains(c)).count() * (factor - 1);
            let expanded_x = (minx..maxx).filter(|c| cols.contains(c)).count() * (factor - 1);

            Some(distance(a, b) + expanded_x + expanded_y)
        })
        .sum();

    format!("{}", sum)
}
fn render(board: &Vec<Vec<Cell>>) {
    println!("\x1Bc");
    for (y, row) in board.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let c = match col {
                Cell::Empty => ".",
                Cell::Galaxy => "#",
            };
            print!("{}", c);
        }
        println!();
    }
}

fn parse2(lines: &Vec<String>) -> (Vec<Vec<Cell>>, (HashSet<usize>, HashSet<usize>)) {
    let board = lines
        .iter()
        .flat_map(|l| {
            vec![l
                .chars()
                .map(|c| {
                    match c {
                        '.' => Some(Cell::Empty),
                        '#' => Some(Cell::Galaxy),
                        _ => None,
                    }
                    .expect("must resolve")
                })
                .collect_vec()]
        })
        .collect_vec();

    let expanded_rows = (0..lines.len())
        .filter(|i| {
            if lines[*i].chars().all(|c| c == '.') {
                true
            } else {
                false
            }
        })
        .collect::<HashSet<usize>>();
    let expanded_cols = (0..lines[0].len())
        .filter(|i| {
            if board.iter().all(|l| l[*i] == Cell::Empty) {
                true
            } else {
                false
            }
        })
        .collect::<HashSet<usize>>();
    (board, (expanded_rows, expanded_cols))
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Cell>> {
    let empty_row: Vec<Cell> = (0..lines[0].len()).map(|_| Cell::Empty).collect_vec();
    let mut board = lines
        .iter()
        .flat_map(|l| {
            if l.chars().all(|c| c == '.') {
                vec![empty_row.clone(), empty_row.clone()]
            } else {
                vec![l
                    .chars()
                    .map(|c| {
                        match c {
                            '.' => Some(Cell::Empty),
                            '#' => Some(Cell::Galaxy),
                            _ => None,
                        }
                        .expect("must resolve")
                    })
                    .collect_vec()]
            }
        })
        .collect_vec();

    let expanded_cols = (0..lines[0].len())
        .filter(|i| {
            if board.iter().all(|l| l[*i] == Cell::Empty) {
                true
            } else {
                false
            }
        })
        .collect_vec();
    board
        .iter()
        .map(|l| {
            l.iter()
                .enumerate()
                .flat_map(|(i, c)| {
                    if expanded_cols.contains(&i) {
                        vec![Cell::Empty, Cell::Empty]
                    } else {
                        vec![*c]
                    }
                })
                .collect_vec()
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    fn sampledata() -> Vec<String> {
        vec![
            "...#......".to_string(),
            ".......#..".to_string(),
            "#.........".to_string(),
            "..........".to_string(),
            "......#...".to_string(),
            ".#........".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            ".......#..".to_string(),
            "#...#.....".to_string(),
        ]
    }

    #[test]
    fn test_part1_sample() -> Result<(), String> {
        assert_eq!("374", part1(&sampledata()));
        Ok(())
    }

    #[rstest]
    #[case(2, "374")]
    #[case(10, "1030")]
    #[case(100, "8410")]
    fn test_part2_sample(#[case] factor: usize, #[case] expected: String) {
        assert_eq!(expected, part2(&sampledata(), factor));
    }
}
