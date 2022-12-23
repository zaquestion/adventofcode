use std::io;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell {
    Null,
    Open,
    Wall,
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point(isize, isize);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

fn siumulate(lines: &Vec<String>, fold: bool) -> usize {
    let mut board: Vec<Vec<Cell>> = Vec::new();
    let maxlen = lines
        .iter()
        .take_while(|l| *l != "")
        .map(|l| l.len())
        .max()
        .unwrap();
    for (row, line) in lines.iter().enumerate() {
        if line == "" {
            break;
        }
        board.push(vec![Cell::Null; maxlen]);
        for (col, c) in line.chars().enumerate() {
            board[row][col] = match c {
                ' ' => Cell::Null,
                '.' => Cell::Open,
                '#' => Cell::Wall,
                _ => Cell::Null,
            }
        }
    }

    let steps = lines
        .last()
        .unwrap()
        .split(char::is_alphabetic)
        .collect::<Vec<_>>();
    let turns = lines
        .last()
        .unwrap()
        .split(char::is_numeric)
        .filter(|&c| c != "")
        .collect::<Vec<_>>();

    let startx = board
        .first()
        .unwrap()
        .iter()
        .position(|c| c == &Cell::Open)
        .unwrap();
    let mut pos = Point(startx as isize, 0 as isize);
    println!("Starting: {:?}", pos);
    let mut facing = Direction::Right;

    //println!("{:?}", pos);

    for (i, step) in steps.iter().enumerate() {
        //println!("STEP: {:?}, DIRECTION: {:?}", step, facing.clone());
        let tostep = step.parse::<usize>().expect("must num");
        for _ in 0..tostep {
            let unit = match facing {
                Direction::Up => Point(0 as isize, -1 as isize),
                Direction::Down => Point(0 as isize, 1 as isize),
                Direction::Left => Point(-1 as isize, 0 as isize),
                Direction::Right => Point(1 as isize, 0 as isize),
            };
            let prevfacing = facing.clone();
            let prevpos = pos.clone();
            if fold {
                (pos, facing) = bounds_or_wrap_3d(
                    &board,
                    pos.clone() + unit.clone(),
                    pos.clone(),
                    facing.clone(),
                )
                .expect("must resolve");
            } else {
                pos = bounds_or_wrap_2d(
                    &board,
                    pos.clone() + unit.clone(),
                    pos.clone(),
                    facing.clone(),
                )
                .expect("must resolve");
            }
            /*
            println!(
                "{:?},{:?},{} -> {:?},{:?},{}",
                prevpos,
                prevfacing,
                get_face(prevpos.clone()).expect("must face"),
                pos.clone(),
                facing,
                get_face(pos.clone()).expect("must face")
            );
            */
        }
        if i < turns.len() {
            let turn = turns[i];
            let prevfacing = facing.clone();
            facing = match facing {
                Direction::Up => match turn {
                    "L" => Some(Direction::Left),
                    "R" => Some(Direction::Right),
                    _ => None,
                },
                Direction::Down => match turn {
                    "L" => Some(Direction::Right),
                    "R" => Some(Direction::Left),
                    _ => None,
                },
                Direction::Left => match turn {
                    "L" => Some(Direction::Down),
                    "R" => Some(Direction::Up),
                    _ => None,
                },
                Direction::Right => match turn {
                    "L" => Some(Direction::Up),
                    "R" => Some(Direction::Down),
                    _ => None,
                },
            }
            .expect("must rotate");
            //println!("prev: {:?}, turn: {}, cur: {:?}", prevfacing, turn, facing);
        };
    }

    let fval = match facing {
        Direction::Up => 3,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Right => 0,
    };
    println!(
        "vals: row: {}, col: {}, fval: {}, facing: {:?}",
        pos.1, pos.0, fval, facing
    );
    (((pos.1 + 1) * 1000) + ((pos.0 + 1) * 4) + fval) as usize
}

fn bounds_or_wrap_2d(
    board: &Vec<Vec<Cell>>,
    nextpos: Point,
    curpos: Point,
    d: Direction,
) -> Option<Point> {
    if nextpos.1 > board.len() as isize - 1
        || (nextpos.1 > 0
            && board[nextpos.1 as usize][curpos.0 as usize] == Cell::Null
            && d == Direction::Down)
    {
        for (y, row) in board.iter().enumerate() {
            println!("CHECK D {:?}", Point(nextpos.0, y as isize));
            if row[nextpos.0 as usize] == Cell::Null {
                continue;
            }
            if row[nextpos.0 as usize] == Cell::Wall {
                return Some(curpos);
            }

            return Some(Point(nextpos.0, y as isize));
        }
    } else if nextpos.1 < 0
        || (nextpos.1 < board.len() as isize - 1
            && board[nextpos.1 as usize][curpos.0 as usize] == Cell::Null
            && d == Direction::Up)
    {
        for (y, row) in board.iter().enumerate().rev() {
            println!("CHECK U {:?}", Point(nextpos.0, y as isize));
            if row[nextpos.0 as usize] == Cell::Null {
                continue;
            }
            if row[nextpos.0 as usize] == Cell::Wall {
                return Some(curpos);
            }

            return Some(Point(nextpos.0, y as isize));
        }
    } else if nextpos.0 > board[nextpos.1 as usize].len() as isize - 1
        || (nextpos.0 > 0
            && board[curpos.1 as usize][nextpos.0 as usize] == Cell::Null
            && d == Direction::Right)
    {
        for (x, c) in board[nextpos.1 as usize].iter().enumerate() {
            println!("CHECK R {:?}", Point(x as isize, nextpos.1));
            if *c == Cell::Null {
                continue;
            }
            if *c == Cell::Wall {
                return Some(curpos);
            }

            return Some(Point(x as isize, nextpos.1));
        }
    } else if nextpos.0 < 0
        || (nextpos.0 < board[nextpos.1 as usize].len() as isize - 1
            && board[curpos.1 as usize][nextpos.0 as usize] == Cell::Null
            && d == Direction::Left)
    {
        for (x, c) in board[nextpos.1 as usize].iter().enumerate().rev() {
            println!("CHECK L {:?}", Point(x as isize, nextpos.1));
            if *c == Cell::Null {
                continue;
            }
            if *c == Cell::Wall {
                return Some(curpos);
            }

            return Some(Point(x as isize, nextpos.1));
        }
    }
    if board[nextpos.1 as usize][nextpos.0 as usize] == Cell::Wall {
        return Some(curpos);
    }

    return Some(nextpos);
}

fn get_face(pos: Point) -> Option<usize> {
    let len = 50 - 1;
    if pos.0 >= 50 && pos.0 <= 50 + len && pos.1 >= 0 && pos.1 <= 0 + len {
        return Some(1);
    } else if pos.0 >= 100 && pos.0 <= 100 + len && pos.1 >= 0 && pos.1 <= 0 + len {
        return Some(2);
    } else if pos.0 >= 50 && pos.0 <= 50 + len && pos.1 >= 50 && pos.1 <= 50 + len {
        return Some(3);
    } else if pos.0 >= 0 && pos.0 <= 0 + len && pos.1 >= 100 && pos.1 <= 100 + len {
        return Some(4);
    } else if pos.0 >= 50 && pos.0 <= 50 + len && pos.1 >= 100 && pos.1 <= 100 + len {
        return Some(5);
    } else if pos.0 >= 0 && pos.0 <= 0 + len && pos.1 >= 150 && pos.1 <= 150 + len {
        return Some(6);
    }

    return None;
}
fn wrap_3d(face: usize, facing: Direction, nextpos: Point) -> Option<(Direction, Point)> {
    match face {
        1 => match facing {
            Direction::Up => Some((Direction::Right, Point(0, nextpos.0 - 50 + 150))), // 6

            Direction::Right => Some((Direction::Right, nextpos)), // 2
            Direction::Down => Some((Direction::Down, nextpos)),   // 3
            Direction::Left => Some((Direction::Right, Point(0, 50 - nextpos.1 + 99))), // 4 // v
        },
        2 => match facing {
            Direction::Up => Some((Direction::Up, Point(nextpos.0 - 100, 199))), // 6
            Direction::Right => Some((Direction::Left, Point(99, 50 - (nextpos.1) + 99))), // 5 //v
            Direction::Down => Some((Direction::Left, Point(99, (nextpos.0 - 100) + 50))), // 3 //v
            Direction::Left => Some((Direction::Left, nextpos)),                 // 1
        },
        3 => match facing {
            Direction::Up => Some((Direction::Up, nextpos)), // 1
            Direction::Right => Some((Direction::Up, Point(nextpos.1 - 50 + 100, 49))), // 2//v
            Direction::Down => Some((Direction::Down, nextpos)), // 5
            Direction::Left => Some((Direction::Down, Point(nextpos.1 - 50, 100))), // 4
        },
        4 => match facing {
            Direction::Up => Some((Direction::Right, Point(50, nextpos.0 + 50))), // 3
            Direction::Right => Some((Direction::Right, nextpos)),                // 5
            Direction::Down => Some((Direction::Down, nextpos)),                  // 6
            Direction::Left => Some((Direction::Right, Point(50, 50 - (nextpos.1 - 100) - 1))), // 1 // v
        },
        5 => match facing {
            Direction::Up => Some((Direction::Up, nextpos)), // 3
            Direction::Right => Some((Direction::Left, Point(149, 50 - (nextpos.1 - 100 - 1) - 1))), //2 //v
            Direction::Down => Some((Direction::Left, Point(49, (nextpos.0 - 50) + 150))), // 6//v
            Direction::Left => Some((Direction::Left, nextpos)),                           // 4
        },
        6 => match facing {
            Direction::Up => Some((Direction::Up, nextpos)), // 4
            Direction::Right => Some((Direction::Up, Point((nextpos.1 - 150) + 50, 149))), // 5 //v
            Direction::Down => Some((Direction::Down, Point(nextpos.0 + 100, 0))), // 2
            Direction::Left => Some((Direction::Down, Point((nextpos.1 - 150) + 50, 0))), // 1
        },
        _ => None,
    }
}
fn bounds_or_wrap_3d(
    board: &Vec<Vec<Cell>>,
    nextpos: Point,
    curpos: Point,
    facing: Direction,
) -> Option<(Point, Direction)> {
    let face = get_face(curpos.clone()).expect("must face");
    let mut nextpos = nextpos.clone();
    let mut facing = facing.clone();
    if (nextpos.1 > board.len() as isize - 1
        || (nextpos.1 > 0
            && board[nextpos.1 as usize][curpos.0 as usize] == Cell::Null
            && facing == Direction::Down))
        || (nextpos.1 < 0
            || (nextpos.1 < board.len() as isize - 1
                && board[nextpos.1 as usize][curpos.0 as usize] == Cell::Null
                && facing == Direction::Up))
        || (nextpos.0 > board[nextpos.1 as usize].len() as isize - 1
            || (nextpos.0 > 0
                && board[curpos.1 as usize][nextpos.0 as usize] == Cell::Null
                && facing == Direction::Right))
        || (nextpos.0 < 0
            || (nextpos.0 < board[nextpos.1 as usize].len() as isize - 1
                && board[curpos.1 as usize][nextpos.0 as usize] == Cell::Null
                && facing == Direction::Left))
    {
        let (prevfacing, prevnextpos) = (facing.clone(), nextpos.clone());
        (facing, nextpos) = wrap_3d(face, facing, nextpos).expect("must resolve");
        println!(
            "BOUNDARY -- {:?},{:?},{:?} -> {:?},{:?},{:?}",
            prevfacing,
            prevnextpos,
            face,
            facing,
            nextpos.clone(),
            get_face(nextpos.clone()).expect("must face")
        );
    }
    if board[nextpos.1 as usize][nextpos.0 as usize] == Cell::Wall {
        return Some((curpos, facing));
    }

    return Some((nextpos, facing));
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    println!("part1: {}", siumulate(&lines, false));
    // WRONG: 78296
    // WRONG: 141180
    // RIGHT: 1484
    println!("part2: {}", siumulate(&lines, true));
    // WRONG: 79358
    // WRONG: 28377
    // WRONG: 134152
    // WRONG: 198097
    // WRONG: 38391
    // RIGHT: 142228
}
