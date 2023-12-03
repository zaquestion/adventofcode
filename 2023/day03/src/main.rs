// Ref: https://lazy.codes/posts/awesome-unstable-rust-features/
#![feature(if_let_guard)]

use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    // WRONG: 520088
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Symbol(char),
    PartNumberPart(usize),
}

fn part1(lines: &Vec<String>) -> String {
    // $ cat input| tr -d '0123456789.\n' | grep -o . | sort -u | tr -d '\n' && echo
    let symbols = "#m%m&*+-/=@$".to_string();
    let board: Vec<Vec<Cell>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    // c if c.is_numeric() => { if let Ok(d) = c.to_string().parse::<usize>() { Some(Cell::PartNumberPart(d)) } else { None } }
                    // This here is the experimental if_let_guard. which we
                    // didn't really actually need/use because we index into the line directly later..
                    // kept because it was the learning undertaken in this challenge
                    c if let Ok(v) = c.to_string().parse::<usize>() => Some(Cell::PartNumberPart(v)),
                    c if symbols.chars().any(|s| s == c) => Some(Cell::Symbol(c)),
                    '.' => Some(Cell::Empty),
                    _ => None,
                })
                .map(|c| c.expect("must resolve"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = (0..board.len())
        .cartesian_product(0..board[0].len())
        .filter(|(y, x)| matches!(board[*y][*x], Cell::Symbol(_)))
        .map(adjacent_part_nums(&board, lines))
        .flat_map(|nums| nums.iter().cloned().collect::<Vec<_>>())
        .sum::<usize>();

    format!("{}", sum)
}

fn part2(lines: &Vec<String>) -> String {
    // $ cat input| tr -d '0123456789.\n' | grep -o . | sort -u | tr -d '\n' && echo
    let symbols = "#m%m&*+-/=@$".to_string();
    let board: Vec<Vec<Cell>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    // c if c.is_numeric() => { if let Ok(d) = c.to_string().parse::<usize>() { Some(Cell::PartNumberPart(d)) } else { None } }
                    // NOTE: to run without experiemental, uncomment the
                    // above and comment the below "if let gaurd". Remove the
                    // if_let_gaurd feature at the top of the file
                    // 
                    // This here is the experimental if_let_guard. which we
                    // didn't really actually need/use because we index into the line directly later..
                    // kept because it was the learning undertaken in this challenge
                    c if let Ok(v) = c.to_string().parse::<usize>() => Some(Cell::PartNumberPart(v)),
                    c if symbols.chars().any(|s| s == c) => Some(Cell::Symbol(c)),
                    '.' => Some(Cell::Empty),
                    _ => None,
                })
                .map(|c| c.expect("must resolve"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let sum = (0..board.len())
        .cartesian_product(0..board[0].len())
        .filter(|(y, x)| matches!(board[*y][*x], Cell::Symbol('*')))
        .map(adjacent_part_nums(&board, lines))
        .filter_map(|nums| {
            if nums.len() == 2 {
                Some(nums.iter().product::<usize>())
            } else {
                None
            }
        })
        .sum::<usize>();

    format!("{}", sum)
}

fn adjacent_part_nums<'a>(
    board: &'a Vec<Vec<Cell>>,
    lines: &'a Vec<String>,
) -> impl Fn((usize, usize)) -> Vec<usize> + 'a {
    |(y, x)| {
        vec![
            (y + 1, x),
            (y - 1, x),
            (y, x - 1),
            (y, x + 1),
            (y + 1, x + 1),
            (y - 1, x + 1),
            (y - 1, x - 1),
            (y + 1, x - 1),
        ]
        .iter()
        .filter(|(y, x)| {
            if *y > board.len() || *x > board[0].len() {
                false
            } else if matches!(board[*y][*x], Cell::PartNumberPart(_)) {
                true
            } else {
                false
            }
        })
        .map(|(y, x)| {
            let startx = (1..=*x)
                .rev()
                .skip_while(|xx| matches!(board[*y][*xx - 1], Cell::PartNumberPart(_)))
                .take(1)
                .exactly_one()
                .unwrap_or_else(|_| 0);
            let endx = (*x..board[0].len())
                .skip_while(|xx| matches!(board[*y][*xx], Cell::PartNumberPart(_)))
                .take(1)
                .exactly_one()
                .unwrap_or_else(|_| board[0].len());
            lines[*y]
                .get(startx..endx)
                .expect("valid slice")
                .to_string()
                .parse::<usize>()
                .expect("must num")
        })
        .unique()
        .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_sample() -> Result<(), String> {
        let input: Vec<String> = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!("4361", part1(&input));
        Ok(())
    }

    #[test]
    fn test_part1_right_most() -> Result<(), String> {
        let input: Vec<String> = vec![
            "467..114.".to_string(),
            "...*.....".to_string(),
            "..35..633".to_string(),
            "......#..".to_string(),
            "617*.....".to_string(),
            ".....+.58".to_string(),
            "..592....".to_string(),
            "......755".to_string(),
            "...$.*...".to_string(),
            ".664.598.".to_string(),
        ];
        assert_eq!("4361", part1(&input));
        Ok(())
    }

    #[test]
    fn test_part2_sample() -> Result<(), String> {
        let input: Vec<String> = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string(),
        ];
        assert_eq!("467835", part2(&input));
        Ok(())
    }

    #[test]
    fn test_part2_right_most() -> Result<(), String> {
        let input: Vec<String> = vec![
            "467..114.".to_string(),
            "...*.....".to_string(),
            "..35..633".to_string(),
            "......#..".to_string(),
            "617*.....".to_string(),
            ".....+.58".to_string(),
            "..592....".to_string(),
            "......755".to_string(),
            "...$.*...".to_string(),
            ".664.598.".to_string(),
        ];
        assert_eq!("467835", part2(&input));
        Ok(())
    }
}
