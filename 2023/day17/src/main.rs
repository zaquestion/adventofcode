use colored::Colorize;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::Add;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

fn parse(lines: &Vec<String>) -> Vec<Vec<usize>> {
    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<usize>().expect("must num"))
                .collect_vec()
        })
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> String {
    let board = parse(lines);
    let rows = board.len();
    let cols = board[0].len();

    let (path, heat_loss) = dijkstra(
        &State::new(Point(0, 0), 1),
        |p| p.successors(&board, 1, 3),
        |p| p.pos == Point(rows as isize - 1, cols as isize - 1),
    )
    .expect("must path");

    let (_, heat_loss2) = dijkstra(
        &State::new(Point(0, 0), 2),
        |p| p.successors(&board, 1, 3),
        |p| p.pos == Point(rows as isize - 1, cols as isize - 1),
    )
    .expect("must path");

    let steps = path
        .iter()
        .map(|s| (s.pos, s.dir))
        .collect::<HashMap<Point, usize>>();
    path.iter()
        .for_each(|p| println!("{:?}, d: {}, s: {}", p.pos, p.dir, p.steps_in_dir));
    render(&board, &steps);
    // println!("{:?}", path);
    format!("{:?}", heat_loss.min(heat_loss2))
}

fn part2(lines: &Vec<String>) -> String {
    let board = parse(lines);

    let rows = board.len();
    let cols = board[0].len();

    let (path, heat_loss) = dijkstra(
        &State::new(Point(0, 0), 1),
        |p| p.successors(&board, 4, 10),
        |p| p.pos == Point(rows as isize - 1, cols as isize - 1),
    )
    .expect("must path");

    let (_, heat_loss2) = dijkstra(
        &State::new(Point(0, 0), 2),
        |p| p.successors(&board, 4, 10),
        |p| p.pos == Point(rows as isize - 1, cols as isize - 1),
    )
    .expect("must path");

    let steps = path
        .iter()
        .map(|s| (s.pos, s.dir))
        .collect::<HashMap<Point, usize>>();
    path.iter()
        .for_each(|p| println!("{:?}, d: {}, s: {}", p.pos, p.dir, p.steps_in_dir));
    render(&board, &steps);
    // println!("{:?}", path);
    format!("{:?}", heat_loss.min(heat_loss2))
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct State {
    pos: Point,
    dir: usize,
    steps_in_dir: usize,
}

impl State {
    fn new(start: Point, d: usize) -> State {
        State {
            pos: start,
            dir: d,
            steps_in_dir: 0,
        }
    }
}
impl State {
    fn successors(
        &self,
        b: &Vec<Vec<usize>>,
        min_step: usize,
        max_step: usize,
    ) -> Vec<(State, usize)> {
        let rows = b.len();
        let cols = b[0].len();
        let mut nextstates: Vec<State> = vec![];
        for next_dir in 0..4 {
            if next_dir == (self.dir + 2) % 4 {
                continue;
            }

            let d = DIRS[next_dir];
            let next_pos = &self.pos + &d;

            if !(next_pos.0 < rows as isize
                && next_pos.1 < cols as isize
                && next_pos.0 >= 0
                && next_pos.1 >= 0)
            {
                continue;
            }

            let next_steps = if next_dir == self.dir {
                self.steps_in_dir + 1
            } else {
                if self.steps_in_dir < min_step - 1 {
                    continue;
                }
                0
            };
            if next_steps > max_step - 1 {
                continue;
            }
            nextstates.push(State {
                pos: next_pos,
                dir: next_dir,
                steps_in_dir: next_steps,
            })
        }
        nextstates
            .into_iter()
            .map(|p| (p.clone(), b[p.pos.0 as usize][p.pos.1 as usize]))
            .collect_vec()
    }
}

fn render(b: &Vec<Vec<usize>>, points: &HashMap<Point, usize>) {
    println!("\x1Bc");
    (0..b.len()).for_each(|y| {
        (0..b[0].len()).for_each(|x| {
            let p = &Point(y as isize, x as isize);
            if points.contains_key(p) {
                let dc = match points.get(p).expect("must exist") {
                    0 => "^",
                    1 => ">",
                    2 => "v",
                    3 => "<",
                    _ => panic!("oops"),
                };
                print!("{}", dc.purple());
            } else {
                print!("{}", b[y][x]);
            }
        });
        println!("");
    });
    println!("");
    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
struct Point(isize, isize);
// type Point = (isize, isize);

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

const DIRS: [Point; 4] = [Point(-1, 0), Point(0, 1), Point(1, 0), Point(0, -1)];

fn min_heat_loss_dp(board: Vec<Vec<usize>>) -> usize {
    let rows = board.len();
    let cols = board[0].len();

    let max_moves = 3;

    let mut dp = vec![vec![vec![vec![usize::MAX; max_moves]; 4]; cols]; rows];

    for dir in 0..4 {
        dp[0][0][dir][0] = 0;
    }

    for y in 0..rows {
        for x in 0..cols {
            for dir in 0..4 {
                for moves in 0..max_moves {
                    if dp[y][x][dir][moves] == usize::MAX {
                        continue;
                    }

                    for new_dir in 0..4 {
                        if new_dir == (dir + 2) % 4 {
                            continue;
                        }

                        let d = DIRS[new_dir];
                        let Point(new_y, new_x) = &Point(y as isize, x as isize) + &d;

                        if !(new_y < rows as isize
                            && new_x < cols as isize
                            && new_y >= 0
                            && new_x >= 0)
                        {
                            continue;
                        }

                        let new_moves = if new_dir == dir { moves + 1 } else { 0 };
                        if new_moves < max_moves {
                            let heat_loss =
                                dp[y][x][dir][moves] + board[new_y as usize][new_x as usize];
                            dp[new_y as usize][new_x as usize][new_dir][new_moves] = dp
                                [new_y as usize][new_x as usize][new_dir][new_moves]
                                .min(heat_loss);
                        }
                    }
                }
            }
        }
    }

    let mut min_loss = usize::MAX;
    for dir in 0..4 {
        for moves in 0..max_moves {
            min_loss = min_loss.min(dp[rows - 1][cols - 1][dir][moves]);
        }
    }
    println!("{:?},", min_loss);
    for dir in 0..4 {
        for moves in 0..max_moves {
            println!(
                "d: {:?}, m: {}, {:?}",
                DIRS[dir], moves, dp[0][6][dir][moves]
            );
        }
    }
    for y in 0..rows {
        for x in 0..cols {
            let mut min_loss = usize::MAX;
            for dir in 0..4 {
                for moves in 0..max_moves {
                    min_loss = min_loss.min(dp[y][x][dir][moves]);
                }
            }
            if min_loss != usize::MAX {
                print!("{:0>3},", min_loss);
            } else {
                print!("{},", "MAX".magenta());
            }
        }
        println!();
    }

    min_loss
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "2413432311323".to_string(),
            "3215453535623".to_string(),
            "3255245654254".to_string(),
            "3446585845452".to_string(),
            "4546657867536".to_string(),
            "1438598798454".to_string(),
            "4457876987766".to_string(),
            "3637877979653".to_string(),
            "4654967986887".to_string(),
            "4564679986453".to_string(),
            "1224686865563".to_string(),
            "2546548887735".to_string(),
            "4322674655533".to_string(),
        ]
    }

    /*
    2>>34^>>>1323
    32v>>>35v5623
    32552456v>>54
    */
    fn sampledata3x3() -> Vec<String> {
        vec![
            "241".to_string(), // stay
            "321".to_string(), // on sep
            "325".to_string(), // lines
        ]
    }
    fn sampledata4x4() -> Vec<String> {
        vec![
            "2413".to_string(),
            "3215".to_string(),
            "3255".to_string(),
            "3446".to_string(),
        ]
    }
    fn sampledata5x5() -> Vec<String> {
        vec![
            "24134".to_string(),
            "32154".to_string(),
            "32552".to_string(),
            "34465".to_string(),
            "45466".to_string(),
        ]
    }
    fn sampledata6x6() -> Vec<String> {
        vec![
            "241343".to_string(),
            "321545".to_string(),
            "325524".to_string(),
            "344658".to_string(),
            "454665".to_string(),
            "143859".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata3x3(), "11")]
    #[case(sampledata4x4(), "21")]
    #[case(sampledata5x5(), "28")]
    #[case(sampledata6x6(), "42")]
    #[case(sampledata(), "102")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    fn sampledata_p2_extra() -> Vec<String> {
        vec![
            "111111111111".to_string(),
            "999999999991".to_string(),
            "999999999991".to_string(),
            "999999999991".to_string(),
            "999999999991".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "94")]
    #[case(sampledata_p2_extra(), "71")] // fails and I don't really get it, but we passed on the real for part 2
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}
