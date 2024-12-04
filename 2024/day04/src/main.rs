use itertools::Itertools;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "18")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "9")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    X,
    M,
    A,
    S,
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Cell>> {
    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'X' => Cell::X,
                    'M' => Cell::M,
                    'A' => Cell::A,
                    'S' => Cell::S,
                    _ => Cell::Empty,
                })
                .collect_vec()
        })
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> String {
    let board = parse(lines);

    let mut num = 0;
    let max_x = board.first().expect("must first").len();
    let max_y = board.len();
    for (y, x) in (0..max_y).cartesian_product(0..max_x) {
        if board[y][x] != Cell::X {
            continue;
        }
        if x + 3 < max_x {
            if board[y][x + 1] == Cell::M
                && board[y][x + 2] == Cell::A
                && board[y][x + 3] == Cell::S
            {
                num += 1;
            }
        }
        if x as isize - 3 >= 0 {
            if board[y][x - 1] == Cell::M
                && board[y][x - 2] == Cell::A
                && board[y][x - 3] == Cell::S
            {
                num += 1;
            }
        }
        if y + 3 < max_y {
            if board[y + 1][x] == Cell::M
                && board[y + 2][x] == Cell::A
                && board[y + 3][x] == Cell::S
            {
                num += 1;
            }
        }
        if y as isize - 3 >= 0 {
            if board[y - 1][x] == Cell::M
                && board[y - 2][x] == Cell::A
                && board[y - 3][x] == Cell::S
            {
                num += 1;
            }
        }

        if y + 3 < max_y && x + 3 < max_x {
            if board[y + 1][x + 1] == Cell::M
                && board[y + 2][x + 2] == Cell::A
                && board[y + 3][x + 3] == Cell::S
            {
                num += 1;
            }
        }
        if y + 3 < max_y && x as isize - 3 >= 0 {
            if board[y + 1][x - 1] == Cell::M
                && board[y + 2][x - 2] == Cell::A
                && board[y + 3][x - 3] == Cell::S
            {
                num += 1;
            }
        }
        if y as isize - 3 >= 0 && x + 3 < max_x {
            if board[y - 1][x + 1] == Cell::M
                && board[y - 2][x + 2] == Cell::A
                && board[y - 3][x + 3] == Cell::S
            {
                num += 1;
            }
        }
        if y as isize - 3 >= 0 && x as isize - 3 >= 0 {
            if board[y - 1][x - 1] == Cell::M
                && board[y - 2][x - 2] == Cell::A
                && board[y - 3][x - 3] == Cell::S
            {
                num += 1;
            }
        }
    }

    format!("{}", num)
}

fn part2(lines: &Vec<String>) -> String {
    let board = parse(lines);

    let mut num = 0;
    let max_x = board.first().expect("must first").len();
    let max_y = board.len();
    for (y, x) in (0..max_y).cartesian_product(0..max_x) {
        if board[y][x] != Cell::M {
            continue;
        }
        if y + 2 < max_y && x + 2 < max_x {
            // M M
            //  A
            // S S
            if board[y][x + 2] == Cell::M
                && board[y + 1][x + 1] == Cell::A
                && board[y + 2][x + 2] == Cell::S
                && board[y + 2][x] == Cell::S
            {
                num += 1;
            }
        }
        if y + 2 < max_y && x + 2 < max_x {
            // M S
            //  A
            // M S
            if board[y][x + 2] == Cell::S
                && board[y + 1][x + 1] == Cell::A
                && board[y + 2][x + 2] == Cell::S
                && board[y + 2][x] == Cell::M
            {
                num += 1;
            }
        }
        if y as isize - 2 >= 0 && x + 2 < max_x {
            // S S
            //  A
            // M M
            if board[y - 2][x] == Cell::S
                && board[y - 1][x + 1] == Cell::A
                && board[y - 2][x + 2] == Cell::S
                && board[y][x + 2] == Cell::M
            {
                num += 1;
            }
        }
        if y + 2 < max_y && x as isize - 2 >= 0 {
            // S M
            //  A
            // S M
            if board[y][x - 2] == Cell::S
                && board[y + 1][x - 1] == Cell::A
                && board[y + 2][x - 2] == Cell::S
                && board[y + 2][x] == Cell::M
            {
                num += 1;
            }
        }
    }

    format!("{}", num)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}
