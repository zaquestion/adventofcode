use colored::Colorize;
use itertools::Itertools;
use std::collections::HashSet;
use std::io;
use std::ops::{Add, Sub};

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    VerticalPipe,
    HorizontalPipe,
    BottomLeftPipe,
    BottomRightPipe,
    TopRightPipe,
    TopLeftPipe,
    Start,
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

fn directions() -> Vec<(Point, Vec<Cell>)> {
    vec![
        (
            Point(0, -1),
            vec![
                Cell::HorizontalPipe,
                Cell::BottomLeftPipe,
                Cell::TopLeftPipe,
                Cell::Start,
            ],
        ),
        (
            Point(0, 1),
            vec![
                Cell::HorizontalPipe,
                Cell::BottomRightPipe,
                Cell::TopRightPipe,
                Cell::Start,
            ],
        ),
        (
            Point(1, 0),
            vec![
                Cell::VerticalPipe,
                Cell::BottomLeftPipe,
                Cell::BottomRightPipe,
                Cell::Start,
            ],
        ),
        (
            Point(-1, 0),
            vec![
                Cell::VerticalPipe,
                Cell::TopRightPipe,
                Cell::TopLeftPipe,
                Cell::Start,
            ],
        ),
    ]
}

fn part1(lines: &Vec<String>) -> String {
    let (board, starting) = parse(lines);

    let dirs = directions();
    let mut cur_pos = starting;
    let mut prev_pos = Point(-1, -1);
    let mut total_steps = 0;

    loop {
        let (cell, next) = dirs
            .iter()
            .find_map(|(dir, valid_cells)| {
                let Point(y, x) = &cur_pos + dir;
                if y < 0 || x < 0 || y > board.len() as isize - 1 || x > board[0].len() as isize - 1
                {
                    return None;
                }
                if Point(y, x) == prev_pos {
                    return None;
                }
                let newcell = board[y as usize][x as usize];

                let opp_dir = &cur_pos - &Point(y, x);
                let (_, opp_valid_cells) = dirs
                    .iter()
                    .find(|(p, _)| p == &opp_dir)
                    .expect("must resolve");

                if opp_valid_cells.contains(&board[cur_pos.0 as usize][cur_pos.1 as usize])
                    && valid_cells.contains(&newcell)
                {
                    Some((newcell, Point(y, x)))
                } else {
                    None
                }
            })
            .expect("must resolve");

        if cell == Cell::Start {
            break;
        }
        total_steps += 1;
        // if total_steps < 100 {
        // println!("{:?} -> {:?} -> {:?}", &prev_pos, &cur_pos, &next);
        // render(&board, &prev_pos, &cur_pos, &next, &total_steps);
        // } else {
        // break;
        // }
        prev_pos = cur_pos;
        cur_pos = next;
    }
    format!("{}", total_steps / 2 + 1)
}

fn part2(lines: &Vec<String>) -> String {
    let (board, starting) = parse(lines);
    let max = Point(board.len() as isize - 1, board[0].len() as isize - 1);

    let dirs = directions();
    let mut loop_nodes: HashSet<Point> = HashSet::new();
    let mut loop_nodes_vec: Vec<Point> = Vec::new();
    loop_nodes_vec.push(starting.clone());
    loop_nodes.insert(starting.clone());
    let mut cur_pos = starting;
    let mut prev_pos = Point(-1, -1);
    loop {
        let (cell, next) = dirs
            .iter()
            .find_map(|(dir, valid_cells)| {
                let Point(y, x) = &cur_pos + dir;
                if y < 0 || x < 0 || y > max.0 || x > max.1 {
                    return None;
                }
                if Point(y, x) == prev_pos {
                    return None;
                }
                let newcell = board[y as usize][x as usize];

                let opp_dir = &cur_pos - &Point(y, x);
                let (_, opp_valid_cells) = dirs
                    .iter()
                    .find(|(p, _)| p == &opp_dir)
                    .expect("must resolve");

                if opp_valid_cells.contains(&board[cur_pos.0 as usize][cur_pos.1 as usize])
                    && valid_cells.contains(&newcell)
                {
                    Some((newcell, Point(y, x)))
                } else {
                    None
                }
            })
            .expect("must resolve");

        if cell == Cell::Start {
            break;
        }
        // if total_steps < 100 {
        // println!("{:?} -> {:?} -> {:?}", &prev_pos, &cur_pos, &next);
        // render(&board, &prev_pos, &cur_pos, &next, &total_steps);
        // } else {
        // break;
        // }
        prev_pos = cur_pos;
        cur_pos = next;
        loop_nodes.insert(cur_pos.clone());
        loop_nodes_vec.push(cur_pos.clone());
    }
    render_loop(&board, &loop_nodes, &loop_nodes_vec);
    let inner_nodes = (0..max.0)
        .cartesian_product(0..max.1)
        .filter(|(y, x)| is_interior_point(&loop_nodes_vec, &Point(*y, *x)))
        .count();
    format!("{}", inner_nodes)
}

fn is_interior_point(loop_nodes: &Vec<Point>, point: &Point) -> bool {
    let mut winding_number = 0;

    for i in 0..loop_nodes.len() {
        let p1 = loop_nodes[i].clone();
        let p2 = loop_nodes[(i + 1) % loop_nodes.len()].clone();

        if p1.1 <= point.1 && p2.1 > point.1 || p1.1 > point.1 && p2.1 <= point.1 {
            let intersect_x = p1.0 + (point.1 - p1.1) * (p2.0 - p1.0) / (p2.1 - p1.1);
            if point.0 < intersect_x {
                winding_number += if p1.1 < p2.1 { 1 } else { -1 };
            }
        }
    }

    winding_number != 0 && !loop_nodes.contains(point)
}

/*
// Stupid odd-even algorithm I couldn't get to work
fn is_interior_point(loop_nodes: &HashSet<Point>, point: &Point, max: &Point) -> bool {
    let Point(y, x) = point;
    let down = (*y..max.0).map(|newy| Point(newy, *x)).collect_vec();
    let up = (0..*y).map(|newy| Point(newy, *x)).collect_vec();
    let right = (*x..max.1).map(|newx| Point(*y, newx)).collect_vec();
    let left = (0..*x).map(|newx| Point(*y, newx)).collect_vec();

    !loop_nodes.contains(point)
        && vec![&up, &down, &left, &right]
            .iter()
            .all(|line| line.iter().any(|p| loop_nodes.contains(p)))
        && !vec![&up, &down, &left, &right]
            .iter()
            .any(|line| line.iter().filter(|p| loop_nodes.contains(p)).count() % 2 == 0)

}
*/

fn render(
    board: &Vec<Vec<Cell>>,
    prev_pos: &Point,
    cur_pos: &Point,
    next: &Point,
    total_steps: &usize,
) {
    println!("\x1Bc");
    for (y, row) in board.iter().enumerate() {
        // print!("{:3}: ", i);
        for (x, col) in row.iter().enumerate() {
            if Point(y as isize, x as isize) == *cur_pos {
                print!("*");
                continue;
            }
            match col {
                Cell::Empty => print!("."),
                Cell::VerticalPipe => print!("|"),
                Cell::HorizontalPipe => print!("-"),
                Cell::BottomLeftPipe => print!("L"),
                Cell::BottomRightPipe => print!("J"),
                Cell::TopRightPipe => print!("7"),
                Cell::TopLeftPipe => print!("F"),
                Cell::Start => print!("S"),
            };
        }
        println!();
    }
    println!(
        "{:?} -> {:?} -> {:?} -- total_steps: {}",
        &prev_pos, &cur_pos, &next, total_steps
    );
}

fn render_loop(board: &Vec<Vec<Cell>>, loop_nodes: &HashSet<Point>, loop_nodes_vec: &Vec<Point>) {
    let max = Point(board.len() as isize - 1, board[0].len() as isize - 1);
    println!("\x1Bc");
    for (y, row) in board.iter().enumerate() {
        // print!("{:3}: ", i);
        for (x, col) in row.iter().enumerate() {
            let c = match col {
                Cell::Empty => ".",
                Cell::VerticalPipe => "|",
                Cell::HorizontalPipe => "-",
                Cell::BottomLeftPipe => "L",
                Cell::BottomRightPipe => "J",
                Cell::TopRightPipe => "7",
                Cell::TopLeftPipe => "F",
                Cell::Start => "S",
            };
            if loop_nodes.contains(&Point(y as isize, x as isize)) {
                print!("{}", c.bold());
            } else if is_interior_point(&loop_nodes_vec, &Point(y as isize, x as isize)) {
                print!("{}", "I".blue());
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn parse(lines: &Vec<String>) -> (Vec<Vec<Cell>>, Point) {
    let mut starting_pos: Option<Point> = None;
    (
        lines
            .iter()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Some(Cell::Empty),
                        '|' => Some(Cell::VerticalPipe),
                        '-' => Some(Cell::HorizontalPipe),
                        'L' => Some(Cell::BottomLeftPipe),
                        'J' => Some(Cell::BottomRightPipe),
                        '7' => Some(Cell::TopRightPipe),
                        'F' => Some(Cell::TopLeftPipe),
                        'S' => {
                            starting_pos = Some(Point(y as isize, x as isize));
                            return Some(Cell::Start);
                        }
                        _ => None,
                    })
                    .map(|cell| cell.expect("must resolve"))
                    .collect_vec()
            })
            .collect_vec(),
        starting_pos.expect("must resolve"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    /*
    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    */
    fn part1_sampledata() -> Vec<String> {
        vec![
            ".....".to_string(),
            ".S-7.".to_string(),
            ".|.|.".to_string(),
            ".L-J.".to_string(),
            ".....".to_string(),
        ]
    }

    fn part1_sampledata2() -> Vec<String> {
        vec![
            "..F7.".to_string(),
            ".FJ|.".to_string(),
            "SJ.L7".to_string(),
            "|F--J".to_string(),
            "LJ...".to_string(),
        ]
    }

    #[rstest]
    #[case(part1_sampledata(), "4")]
    #[case(part1_sampledata2(), "8")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    fn part2_sampledata() -> Vec<String> {
        vec![
            "...........".to_string(),
            ".S-------7.".to_string(),
            ".|F-----7|.".to_string(),
            ".||.....||.".to_string(),
            ".||.....||.".to_string(),
            ".|L-7.F-J|.".to_string(),
            ".|..|.|..|.".to_string(),
            ".L--J.L--J.".to_string(),
            "...........".to_string(),
        ]
    }

    fn part2_sampledata2() -> Vec<String> {
        vec![
            "..........".to_string(),
            ".S------7.".to_string(),
            ".|F----7|.".to_string(),
            ".||....||.".to_string(),
            ".||....||.".to_string(),
            ".|L-7F-J|.".to_string(),
            ".|..||..|.".to_string(),
            ".L--JL--J.".to_string(),
            "..........".to_string(),
        ]
    }

    fn part2_sampledata3() -> Vec<String> {
        vec![
            ".F----7F7F7F7F-7....".to_string(),
            ".|F--7||||||||FJ....".to_string(),
            ".||.FJ||||||||L7....".to_string(),
            "FJL7L7LJLJ||LJ.L-7..".to_string(),
            "L--J.L7...LJS7F-7L7.".to_string(),
            "....F-J..F7FJ|L7L7L7".to_string(),
            "....L7.F7||L7|.L7L7|".to_string(),
            ".....|FJLJ|FJ|F7|.LJ".to_string(),
            "....FJL-7.||.||||...".to_string(),
            "....L---J.LJ.LJLJ...".to_string(),
        ]
    }

    fn part2_sampledata4() -> Vec<String> {
        vec![
            "FF7FSF7F7F7F7F7F---7".to_string(),
            "L|LJ||||||||||||F--J".to_string(),
            "FL-7LJLJ||||||LJL-77".to_string(),
            "F--JF--7||LJLJ7F7FJ-".to_string(),
            "L---JF-JLJ.||-FJLJJ7".to_string(),
            "|F|F-JF---7F7-L7L|7|".to_string(),
            "|FFJF7L7F-JF7|JL---7".to_string(),
            "7-L-JL7||F7|L7F-7F7|".to_string(),
            "L.L7LFJ|||||FJL7||LJ".to_string(),
            "L7JLJL-JLJLJL--JLJ.L".to_string(),
        ]
    }

    #[rstest]
    #[case(part2_sampledata(), "4")]
    #[case(part2_sampledata2(), "4")]
    #[case(part2_sampledata3(), "8")]
    #[case(part2_sampledata4(), "10")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}
