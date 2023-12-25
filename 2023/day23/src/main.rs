use colored::Colorize;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Add;
use std::{collections::HashSet, io};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "#.#####################".to_string(),
            "#.......#########...###".to_string(),
            "#######.#########.#.###".to_string(),
            "###.....#.>.>.###.#.###".to_string(),
            "###v#####.#v#.###.#.###".to_string(),
            "###.>...#.#.#.....#...#".to_string(),
            "###v###.#.#.#########.#".to_string(),
            "###...#.#.#.......#...#".to_string(),
            "#####.#.#.#######.#.###".to_string(),
            "#.....#.#.#.......#...#".to_string(),
            "#.#####.#.#.#########v#".to_string(),
            "#.#...#...#...###...>.#".to_string(),
            "#.#.#v#######v###.###v#".to_string(),
            "#...#.>.#...>.>.#.###.#".to_string(),
            "#####v#.#.###v#.#.###.#".to_string(),
            "#.....#...#...#.#.#...#".to_string(),
            "#.#########.###.#.#.###".to_string(),
            "#...###...#...#...#.###".to_string(),
            "###.###.#.###v#####v###".to_string(),
            "#...#...#.#.>.>.#.>.###".to_string(),
            "#.###.###.#.###.#.#v###".to_string(),
            "#.....###...###...#...#".to_string(),
            "#####################.#".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "94")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), Point(0,1), Point(1,0), (16, Some(vec![(Point(5, 4), Point(0, 1)), (Point(6, 3), Point(1, 0))])))]
    #[case(sampledata(), Point(20,19), Point(1,0), (4, Some(vec![(Point(22, 21), Point(1, 0))])))]
    fn test_walk_to_fork_or_end(
        #[case] input: Vec<String>,
        #[case] start: Point,
        #[case] dir: Point,
        #[case] expected: (usize, Option<Vec<(Point, Point)>>),
    ) {
        let (board, _) = parse(&input);
        assert_eq!(expected, walk_to_fork_or_end(&board, &start, &dir));
    }

    #[rstest]
    #[case(sampledata(), "154")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Path,
    Forest,
    SlopeUp,
    SlopeRight,
    SlopeDown,
    SlopeLeft,
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

fn parse(lines: &Vec<String>) -> (Vec<Vec<Cell>>, (Point, Point)) {
    let b = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    match c {
                        '.' => Some(Cell::Path),
                        '#' => Some(Cell::Forest),
                        '^' => Some(Cell::SlopeUp),
                        '>' => Some(Cell::SlopeRight),
                        'v' => Some(Cell::SlopeDown),
                        '<' => Some(Cell::SlopeLeft),
                        _ => None,
                    }
                    .expect("must resolve")
                })
                .collect_vec()
        })
        .collect_vec();
    let start_end = (
        b.first()
            .expect("must first")
            .iter()
            .enumerate()
            .find_map(|(x, &c)| {
                if Cell::Path == c {
                    Some(Point(0, x as isize))
                } else {
                    None
                }
            })
            .expect("must find"),
        b.last()
            .expect("must first")
            .iter()
            .enumerate()
            .find_map(|(x, &c)| {
                if Cell::Path == c {
                    Some(Point(b.len() as isize - 1, x as isize))
                } else {
                    None
                }
            })
            .expect("must find"),
    );
    (b, start_end)
}

fn part1(lines: &Vec<String>) -> String {
    let (board, (start_p, end_p)) = parse(lines);

    format!("{}", walk(board, start_p, end_p, false))
}

fn part2(lines: &Vec<String>) -> String {
    let (board, (start_p, end_p)) = parse(lines);

    format!("{}", walk_pt2(board, start_p, end_p))
    // format!("{}", walk(board, end_p, start_p, true))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: Point,
    dir: Point,
    forks: Vec<Point>,
    walked: HashSet<Point>,
    steps: usize,
}

impl State {
    fn new(p: Point, d: Point) -> State {
        State {
            walked: HashSet::from([p.clone()]),
            forks: Vec::new(),
            pos: p,
            dir: d,
            steps: 0,
        }
    }
}

const DIRS: [Point; 4] = [Point(-1, 0), Point(0, 1), Point(1, 0), Point(0, -1)];

fn walk_to_fork_or_end(
    board: &Vec<Vec<Cell>>,
    start_pos: &Point,
    start_dir: &Point,
) -> (usize, Option<Vec<(Point, Point)>>) {
    let mut start = State::new(start_pos.clone(), start_dir.clone());
    start.walked.insert(start_pos.clone());
    start
        .walked
        .insert(start_pos + &Point(start_dir.0 * -1, start_dir.1 * -1));

    let mut steps = 0;
    let mut next: Vec<State> = vec![start.clone()];
    while !next.is_empty() {
        let nextnext: Vec<_> = next
            .iter()
            .flat_map(|cur| {
                let pts = available_steps(board, &cur)
                    .iter()
                    .filter_map(|(p, d)| {
                        let mut newstate = cur.clone();
                        newstate.walked.insert(p.clone());
                        match board[p.0 as usize][p.1 as usize] {
                            Cell::Forest => panic!("oops"),
                            _ => {
                                newstate.pos = p.clone();
                                newstate.dir = d.clone();
                                Some(newstate)
                            }
                        }
                    })
                    .collect_vec();
                pts
            })
            .collect_vec();
        steps += 1;
        if nextnext.len() > 1 {
            return (
                steps,
                Some(
                    nextnext
                        .iter()
                        .map(|s| (s.pos.clone(), s.dir.clone()))
                        .collect_vec(),
                ),
            );
        } else if nextnext.len() == 0 {
            if next.len() > 1 {
                panic!("only expecting 1 value in this case");
            }
            return (
                steps - 1,
                Some(
                    next.iter()
                        .map(|s| (s.pos.clone(), s.dir.clone()))
                        .collect_vec(),
                ),
            );
        }
        next = nextnext;
    }
    (steps, None)
}

fn available_steps(board: &Vec<Vec<Cell>>, cur: &State) -> Vec<(Point, Point)> {
    DIRS.iter()
        .filter_map(|d| {
            // last fork leads to end, if you don't go down there is
            // no way to reach it
            if cur.pos == Point(127, 137) {
                if *d == Point(1, 0) {
                    Some((&cur.pos + d, d.clone()))
                } else {
                    None
                }
            } else {
                Some((&cur.pos + d, d.clone()))
            }
        })
        .filter_map(|(next, d)| {
            if next.0 < 0
                || next.0 > board.len() as isize - 1
                || next.1 < 0
                || next.1 > board.first().expect("must first").len() as isize - 1
                || cur.walked.contains(&next)
                || board[next.0 as usize][next.1 as usize] == Cell::Forest
            {
                None
            } else {
                Some((next.clone(), d))
            }
        })
        .collect_vec()
}

#[allow(dead_code)]
fn render(b: &Vec<Vec<Cell>>, walked: &HashSet<Point>) {
    // println!("\x1Bc");
    (0..b.len()).for_each(|y| {
        (0..b[0].len()).for_each(|x| {
            let c = match b[y][x] {
                Cell::Path => ".",
                Cell::Forest => "#",
                Cell::SlopeUp => "^",
                Cell::SlopeRight => ">",
                Cell::SlopeLeft => "<",
                Cell::SlopeDown => "v",
            };
            if walked.contains(&Point(y as isize, x as isize)) {
                print!("{}", c.purple());
            } else {
                print!("{}", c);
            }
        });
        println!("");
    });
    println!("");
    // std::thread::sleep(std::time::Duration::from_millis(200));
}

fn walk_pt2(board: Vec<Vec<Cell>>, start_pos: Point, end_pos: Point) -> usize {
    let start = State::new(start_pos.clone(), DIRS[2].clone());

    let mut next: Vec<State> = vec![start.clone()];
    let mut longest: usize = 0;
    let mut forks: HashMap<(Point, Point), (usize, Vec<(Point, Point)>)> = HashMap::new();
    while !next.is_empty() {
        let nextnext: Vec<State> = next
            .drain(0..)
            .flat_map(|cur| {
                let (steps, pts) = if let Some((steps, pts)) =
                    forks.get(&(cur.pos.clone(), cur.dir.clone()))
                {
                    (steps.to_owned(), pts.to_owned())
                } else {
                    let (steps, Some(pts)) = walk_to_fork_or_end(&board, &cur.pos, &cur.dir) else {
                        panic!("expected fork or end");
                    };
                    (steps, pts)
                };
                forks.insert((cur.pos.clone(), cur.dir.clone()), (steps, pts.clone()));

                // render(&board, &cur.walked);
                if pts.len() == 1 && pts[0].0 == end_pos.clone() {
                    if cur.steps + steps > longest {
                        longest = cur.steps + steps;
                    }
                    vec![]
                } else {
                    pts.iter()
                        .filter_map(|(p, d)| {
                            if !cur.walked.contains(p)
                                && !cur.walked.contains(&(p + &Point(d.0 * -1, d.1 * -1)))
                            {
                                let mut newstate = cur.clone();
                                newstate.pos = p.clone();
                                newstate.dir = d.clone();
                                newstate.steps += steps;
                                newstate.walked.insert(p.clone());
                                newstate.walked.insert(p + &Point(d.0 * -1, d.1 * -1));
                                Some(newstate)
                            } else {
                                None
                            }
                        })
                        .collect_vec()
                }
            })
            .collect_vec();
        // println!("longest: {}, candidates: {}", longest, nextnext.len());
        next = nextnext;
    }
    // dbg!(&start_pos, &end_pos);
    longest
}

fn walk(board: Vec<Vec<Cell>>, start_pos: Point, end_pos: Point, part2: bool) -> usize {
    let start = State::new(start_pos.clone(), DIRS[2].clone());

    let mut next: Vec<State> = vec![start.clone()];
    let mut longest: usize = 0;

    while !next.is_empty() {
        let nextnext: Vec<_> = next
            .drain(0..)
            .flat_map(|cur| {
                let mut pts = if !part2 {
                    match board[cur.pos.0 as usize][cur.pos.1 as usize] {
                        Cell::SlopeUp => {
                            vec![&cur.pos + &Point(-1, 0)]
                        }
                        Cell::SlopeRight => {
                            vec![&cur.pos + &Point(0, 1)]
                        }
                        Cell::SlopeDown => {
                            vec![&cur.pos + &Point(1, 0)]
                        }
                        Cell::SlopeLeft => {
                            vec![&cur.pos + &Point(0, -1)]
                        }
                        Cell::Forest => panic!("oops"),
                        _ => DIRS.iter().map(|d| &cur.pos + d).collect_vec(),
                    }
                } else {
                    DIRS.iter()
                        .filter_map(|d| {
                            if cur.pos == Point(127, 137) {
                                if *d == Point(1, 0) {
                                    Some(&cur.pos + d)
                                } else {
                                    None
                                }
                            } else {
                                Some(&cur.pos + d)
                            }
                        })
                        .collect_vec()
                }
                .iter()
                .filter_map(|next| {
                    if next.0 < 0
                        || next.0 > board.len() as isize - 1
                        || next.1 < 0
                        || next.1 > board.first().expect("must first").len() as isize - 1
                        || cur.walked.contains(&next)
                        || board[next.0 as usize][next.1 as usize] == Cell::Forest
                    {
                        None
                    } else {
                        Some(next.clone())
                    }
                })
                .filter_map(|p| {
                    let mut newstate = cur.clone();
                    newstate.walked.insert(p.clone());
                    if p == end_pos {
                        if cur.walked.len() > longest {
                            // now includes end pos
                            longest = cur.walked.len();
                        }
                        None
                    } else {
                        match board[p.0 as usize][p.1 as usize] {
                            Cell::Forest => panic!("oops"),
                            _ => {
                                newstate.pos = p;
                                Some(newstate)
                            }
                        }
                    }
                })
                .collect_vec();
                let plen = pts.len();
                pts.iter_mut().for_each(|s| {
                    if plen > 1 {
                        s.forks.push(cur.pos.clone());
                    }
                });
                pts
            })
            .collect();
        next = nextnext;
    }
    longest
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    // Wrong: 4942 (too low)
    // Wrong: 5578 (too low)
    println!("part 2: {:?}", part2(&lines));
}
