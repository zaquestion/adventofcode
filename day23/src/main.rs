use std::io;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell {
    Elf,
    ImobileElf,
    Proposal(usize, usize),
    Open,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    N,
    S,
    W,
    E,
}

fn simulate(lines: &Vec<String>, rounds: usize, part2: bool) -> usize {
    let mut board: Vec<Vec<Cell>> = Vec::new();
    for line in lines {
        board.push(
            line.chars()
                .map(|c| match c {
                    '.' => Some(Cell::Open),
                    '#' => Some(Cell::Elf),
                    _ => None,
                })
                .map(|c| c.expect("must resolve"))
                .collect::<Vec<_>>(),
        );
    }
    let mut moves = vec![Direction::N, Direction::S, Direction::W, Direction::E];
    println!("Start!!!");
    surround(&mut board, 1);
    show(&board);

    for r in 1..=rounds {
        for x in 1..board[0].len() - 1 {
            for y in 1..board.len() - 1 {
                if board[y][x] != Cell::Elf {
                    continue;
                }
                if check_border(&board, x, y) {
                    board[y][x] = Cell::ImobileElf;
                    continue;
                }
                let mut proposed: bool = false;
                for m in moves.iter() {
                    let yoff: isize = match m {
                        Direction::N => -1,
                        Direction::S => 1,
                        _ => 0,
                    };
                    let xoff: isize = match m {
                        Direction::W => -1,
                        Direction::E => 1,
                        _ => 0,
                    };
                    //println!("MOVE: {:?}, xoff: {}, yoff: {}", m, xoff, yoff);
                    if yoff != 0 {
                        let newy = (y as isize + yoff) as usize;
                        if [-1, 0, 1].iter().all(|dx| {
                            let c = &board[newy][(x as isize + dx) as usize];
                            *c != Cell::Elf && *c != Cell::ImobileElf
                        }) {
                            if let Cell::Proposal(ox, oy) = board[newy][x] {
                                //neither is allowed to
                                //move in this case
                                board[newy][x] = Cell::Open;
                                board[y][x] = Cell::ImobileElf;
                                board[oy][ox] = Cell::ImobileElf;
                            } else {
                                board[newy][x] = Cell::Proposal(x, y);
                                proposed = true;
                            }
                            break;
                        }
                    } else if xoff != 0 {
                        let newx = (x as isize + xoff) as usize;
                        if [-1, 0, 1].iter().all(|dy| {
                            let c = &board[(y as isize + dy) as usize][newx];
                            *c != Cell::Elf && *c != Cell::ImobileElf
                        }) {
                            if let Cell::Proposal(ox, oy) = board[y][newx] {
                                //neither is allowed to
                                //move in this case
                                board[y][newx] = Cell::Open;
                                board[y][x] = Cell::ImobileElf;
                                board[oy][ox] = Cell::ImobileElf;
                            } else {
                                board[y][newx] = Cell::Proposal(x, y);
                                proposed = true;
                            }
                            break;
                        }
                    }
                }
                if !proposed {
                    board[y][x] = Cell::ImobileElf
                }
            }
        }
        //println!("PROPOSALS");
        //show(&board);
        if part2
            && board
                .iter()
                .all(|r| r.iter().all(|c| *c == Cell::Open || *c == Cell::ImobileElf))
        {
            return r;
        }

        for x in 0..board[0].len() {
            for y in 0..board.len() {
                board[y][x] = match board[y][x] {
                    Cell::Elf => Cell::Open,
                    Cell::ImobileElf => Cell::Elf,
                    Cell::Proposal(..) => Cell::Elf,
                    Cell::Open => Cell::Open,
                }
            }
        }

        println!("Round: {}", r);
        enclose(&mut board);
        show(&board);
        moves.rotate_left(1);
        surround(&mut board, 1);
    }

    println!("FINAL!!!");
    enclose(&mut board);
    show(&board);
    board
        .iter()
        .map(|r| r.iter().filter(|&c| *c == Cell::Open).count())
        .sum::<usize>()
}

fn check_border(board: &Vec<Vec<Cell>>, x: usize, y: usize) -> bool {
    [-1isize, 1].iter().all(|d| {
        (matches!(board[y][(x as isize + d) as usize], Cell::Open)
            || matches!(board[y][(x as isize + d) as usize], Cell::Proposal(..)))
            && (matches!(board[(y as isize + d) as usize][x], Cell::Open)
                || matches!(board[(y as isize + d) as usize][x], Cell::Proposal(..)))
            && (matches!(
                board[(y as isize + d) as usize][(x as isize + d) as usize],
                Cell::Open
            ) || matches!(
                board[(y as isize + d) as usize][(x as isize + d) as usize],
                Cell::Proposal(..)
            ))
            && (matches!(
                board[(y as isize + d) as usize][(x as isize - d) as usize],
                Cell::Open
            ) || matches!(
                board[(y as isize + d) as usize][(x as isize - d) as usize],
                Cell::Proposal(..)
            ))
    })
}
fn show(board: &Vec<Vec<Cell>>) {
    for row in board.iter() {
        println!(
            "{}",
            row.iter()
                .map(|c| match c {
                    Cell::Open => '.',
                    Cell::Elf => '#',
                    Cell::ImobileElf => 'I',
                    Cell::Proposal(..) => 'P',
                })
                .collect::<String>()
        );
    }
    println!();
}
fn surround(board: &mut Vec<Vec<Cell>>, n: usize) {
    for _ in 0..n {
        board.insert(0, vec![Cell::Open; board[0].len()]);
        board.push(vec![Cell::Open; board[0].len()]);
        for row in board.iter_mut() {
            row.insert(0, Cell::Open);
            row.push(Cell::Open);
        }
    }
}
fn enclose(board: &mut Vec<Vec<Cell>>) {
    let mut mutated = true;
    while mutated {
        mutated = false;
        if board[0].iter().all(|c| *c == Cell::Open) {
            board.remove(0);
            mutated = true;
        }
        if board[board.len() - 1].iter().all(|c| *c == Cell::Open) {
            board.remove(board.len() - 1);
            mutated = true;
        }
        if (0..board.len()).all(|i| board[i][0] == Cell::Open) {
            for i in 0..board.len() {
                board[i].remove(0);
            }
            mutated = true;
        }
        if (0..board.len()).all(|i| board[i][board[i].len() - 1] == Cell::Open) {
            for i in 0..board.len() {
                let len = board[i].len() - 1;
                board[i].remove(len);
            }
            mutated = true;
        }
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    println!("part1: {}", simulate(&lines, 10, false));
    println!("part2: {}", simulate(&lines, (2 as usize).pow(16), true));
}
