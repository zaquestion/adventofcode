use std::cmp::min;
use std::collections::HashSet;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    Blizzard(Vec<Direction>),
    Open,
    Wall,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point(isize, isize);

#[derive(Clone, PartialEq, Eq)]
struct State {
    path: Vec<Point>,
}

impl State {
    fn new(p: Point) -> State {
        State { path: vec![p] }
    }
}

fn wrap(board: &Vec<Vec<Cell>>, (x, y): (usize, usize)) -> (usize, usize) {
    if x > board[0].len() - 2 {
        return (1usize, y);
    } else if x < 1 {
        return (board[0].len() - 2, y);
    } else if y > board.len() - 2 {
        return (x, 1usize);
    } else if y < 1 {
        return (x, board.len() - 2);
    }
    (x, y)
}

fn tick(board: &Vec<Vec<Cell>>) -> (Vec<Vec<Cell>>, HashSet<(usize, usize)>) {
    let mut new = board.clone();
    for y in 1..board.len() - 1 {
        for x in 1..board[0].len() - 1 {
            new[y][x] = Cell::Open;
        }
    }
    let mut blizzards = HashSet::new();
    for y in 1..board.len() - 1 {
        for x in 1..board[0].len() - 1 {
            if let Cell::Blizzard(s) = &board[y][x] {
                for t in s.iter() {
                    let (dx, dy) = match t {
                        Direction::Up => (0, -1isize),
                        Direction::Down => (0, 1isize),
                        Direction::Left => (-1isize, 0),
                        Direction::Right => (1isize, 0),
                    };
                    let (nx, ny) = wrap(
                        &board,
                        ((x as isize + dx) as usize, (y as isize + dy) as usize),
                    );
                    if let Cell::Blizzard(ref mut s) = new[ny][nx] {
                        s.push(t.clone());
                        blizzards.insert((nx, ny));
                    } else if matches!(new[ny][nx], Cell::Open) {
                        new[ny][nx] = Cell::Blizzard(vec![t.clone()]);
                        blizzards.insert((nx, ny));
                    }
                }
            } else {
                continue;
            }
        }
    }
    (new, blizzards)
}

fn walk(board: Vec<Vec<Cell>>, startpos: Point, end: &Point) -> (isize, Vec<Vec<Cell>>) {
    /*
    println!("{}", stringify_board(&board));
    println!();
    let mut nextboard = board;
    for _ in 0..5 {
        let new = tick(&nextboard);
        nextboard = new.clone();
        println!("{}", stringify_board(&nextboard));
        println!();
    }
    */

    let start = State::new(startpos.clone());

    let mut next: Vec<State> = vec![start.clone()];
    let mut board = board.clone();
    let mut last_best_path = start.clone();

    for m in 1..usize::MAX {
        //println!("ROUND: {}", m);
        let mut nextnext: Vec<State> = Vec::new();
        let (nextboard, blizzards) = tick(&board).clone();
        //println!("{}", stringify_board(&nextboard.clone()));
        for state in next.iter_mut() {
            let Point(x, y) = state.path.last().unwrap();
            // end point
            if dist(&Point(*x, *y), end) == 0 {
                /*
                if min_safe_path.path.len() == 1 {
                    min_safe_path = state.clone()
                } else if state.path.len() < min_safe_path.path.len() {
                    min_safe_path = state.clone()
                }
                */
                //return state.path.len() as isize - 1isize;
                return (m as isize - 1, board.clone());
            }
            for (dx, dy) in [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = ((*x as isize + dx) as usize, (*y as isize + dy) as usize);
                if (nx == 1 && ny == 0)
                    || (nx as usize == board[0].len() - 2 && ny as usize == board.len() - 1)
                {
                } else if nx > board[0].len() - 2 {
                    continue;
                } else if nx < 1 {
                    continue;
                } else if ny > board.len() - 2 {
                    continue;
                } else if ny < 1 {
                    continue;
                }
                if blizzards.contains(&(nx, ny)) {
                    continue;
                }
                let mut new = state.clone();
                new.path.push(Point(nx as isize, ny as isize));
                nextnext.push(new);
                /*
                match nextboard[ny][nx] {
                    Cell::Wall => continue,
                    Cell::Blizzard(_) => continue,
                    Cell::Open => {
                        let mut new = state.clone();
                        new.path.push(Point(nx as isize, ny as isize));
                        nextnext.push(new);
                    }
                }
                    */
            }
        }
        next = nextnext;
        board = nextboard.clone();
        next.sort_by(|a, b| {
            if dist(a.path.last().unwrap(), &end) > dist(b.path.last().unwrap(), &end) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });
        next.dedup_by(|a, b| a.path.last().unwrap() == b.path.last().unwrap() || a.path == b.path);
        next.retain(|s| {
            let sdist = dist(s.path.last().unwrap(), &end);
            sdist < dist(last_best_path.path.last().unwrap(), &end) + 14
        });
        next.resize(
            min((2 as usize).pow(18), next.len()),
            State::new(startpos.clone()),
        );
        if next.len() > 0 {
            last_best_path = next[0].clone();
            println!(
                "Closest Path: {:?}, dist: {}, opts: {}",
                next[0].path,
                dist(next[0].path.last().unwrap(), &end),
                next.len()
            );
        } else {
            break;
        }
    }
    (-1, Vec::new())
}
fn dist(s: &Point, b: &Point) -> isize {
    (s.0.abs_diff(b.0) + s.1.abs_diff(b.1)) as isize
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let board = parse(&lines);
    let start = Point(1, 0);
    let end = Point(board[0].len() as isize - 2, board.len() as isize - 1);
    println!("End: {},{}", board[0].len() - 2, board.len() - 1);
    //println!("part1: {}", walk(board.clone(), &start.clone(), &end.clone()).0);

    let (there, board) = walk(board.clone(), start.clone(), &end);
    let (back, board) = walk(board.clone(), end.clone(), &start);
    let (thereagain, _) = walk(board.clone(), start.clone(), &end);
    println!("part2 segs: {} -> {} -> {}", there, back, thereagain);
    println!("part2: {}", there + back + thereagain);
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Cell>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Some(Cell::Open),
                    '#' => Some(Cell::Wall),
                    '<' => Some(Cell::Blizzard(vec![Direction::Left])),
                    '>' => Some(Cell::Blizzard(vec![Direction::Right])),
                    '^' => Some(Cell::Blizzard(vec![Direction::Up])),
                    'v' => Some(Cell::Blizzard(vec![Direction::Down])),
                    _ => None,
                })
                .map(|c| c.expect("must resolve"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn stringify_board(board: &Vec<Vec<Cell>>) -> String {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match c {
                    Cell::Open => ".".to_string(),
                    Cell::Wall => "#".to_string(),
                    Cell::Blizzard(s) => {
                        if s.len() > 1 {
                            s.len().to_string()
                        } else {
                            match s.first().unwrap() {
                                Direction::Up => "^".to_string(),
                                Direction::Down => "v".to_string(),
                                Direction::Left => "<".to_string(),
                                Direction::Right => ">".to_string(),
                            }
                        }
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
