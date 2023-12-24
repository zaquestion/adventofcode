use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use itertools::assert_equal;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::mem::swap;
use std::ops::Add;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "...........".to_string(),
            ".....###.#.".to_string(),
            ".###.##..#.".to_string(),
            "..#.#...#..".to_string(),
            "....#.#....".to_string(),
            ".##..S####.".to_string(),
            ".##..#...#.".to_string(),
            ".......##..".to_string(),
            ".##.#.####.".to_string(),
            ".##..##.##.".to_string(),
            "...........".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), 6, "16")]
    #[case(sampledata(), 10000, "42")] // sat steps 13
    fn test_part1_sample(
        #[case] input: Vec<String>,
        #[case] steps: usize,
        #[case] expected: String,
    ) {
        assert_eq!(expected, part1(&input, steps));
    }

    #[rstest]
    #[case(sampledata(), 6, "16")]
    #[case(sampledata(), 10, "50")]
    #[case(sampledata(), 50, "1594")]
    #[case(sampledata(), 100, "6536")]
    #[case(sampledata(), 500, "167004")]
    // #[case(sampledata(), 1000, "668697")]
    // #[case(sampledata(), 5000, "16733044")]
    fn test_part2_sample(
        #[case] input: Vec<String>,
        #[case] steps: usize,
        #[case] expected: String,
    ) {
        assert_eq!(expected, part2(&input, steps));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Plot,
    Rock,
    Start,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Ord, PartialOrd)]
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

fn parse(lines: &Vec<String>) -> (Vec<Vec<Cell>>, Point) {
    let mut start: Option<Point> = None;
    (
        lines
            .iter()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        match c {
                            '.' => Some(Cell::Plot),
                            '#' => Some(Cell::Rock),
                            'S' => {
                                start = Some(Point(y as isize, x as isize));
                                Some(Cell::Plot)
                            }
                            _ => None,
                        }
                        .expect("must resolve")
                    })
                    .collect_vec()
            })
            .collect_vec(),
        start.expect("must find start point"),
    )
}

const DIRS: [Point; 4] = [Point(-1, 0), Point(0, 1), Point(1, 0), Point(0, -1)];

fn part1(lines: &Vec<String>, steps: usize) -> String {
    let (board, start) = parse(lines);

    let (uniq_plots, _) = step_until_staturated(&board, start, steps, 0);
    format!("{}", uniq_plots)
}

fn step_until_staturated(
    board: &Vec<Vec<Cell>>,
    start: Point,
    steps: usize,
    offset: usize,
) -> (usize, usize) {
    let mut seen: HashSet<Point> = HashSet::new();
    let mut opts: HashSet<Point> = HashSet::new();
    let mut starts: HashSet<Point> = HashSet::from([start]);
    // let mut starts: Vec<Point> = vec![start];
    for i in (offset..steps).progress() {
        // let mut nextstarts = Vec::new();
        let mut nextstarts = starts
            .drain()
            .flat_map(|cur| {
                // if i % 2 == 0 {
                // opts.insert(cur.clone());
                // }
                DIRS.iter()
                    .filter_map(|d| {
                        let next = &cur + &d;
                        if next.0 < 0
                            || next.0 > board.len() as isize - 1
                            || next.1 < 0
                            || next.1 > board.first().expect("must first").len() as isize - 1
                            || seen.contains(&next)
                            || board[next.0 as usize][next.1 as usize] == Cell::Rock
                        {
                            None
                        } else {
                            // seen.insert(next.clone());
                            Some(next.clone())
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            // .unique()
            .collect::<HashSet<Point>>();
        // if i % 2 == 1 {
        // opts.extend(nextstarts.clone())
        // }

        if nextstarts.len() == 0 {
            println!();
            println!(
                "max_y: {:?}, max_x: {:?}, max_c: {:?}",
                seen.iter().max_by_key(|Point(y, x)| y.abs()),
                seen.iter().max_by_key(|Point(y, x)| x.abs()),
                seen.iter().max_by_key(|Point(y, x)| x.abs() + y.abs())
            );
            return (opts.len(), i - offset - 1);
        }
        swap(&mut starts, &mut nextstarts);
    }

    (starts.len(), 0)
    // (opts.len(), 0)
}

fn step_until_staturated_wrapped(
    board: &Vec<Vec<Cell>>,
    start: Point,
    steps: usize,
    offset: usize,
    steps_over_bounds: usize,
) -> (usize, usize) {
    let mut seen: HashSet<Point> = HashSet::new();
    let mut opts: HashSet<Point> = HashSet::new();
    let mut starts: HashSet<Point> = HashSet::from([start]);
    // let mut starts: Vec<Point> = vec![start];
    for i in (offset..steps).progress() {
        // let mut nextstarts = Vec::new();
        let mut nextstarts = starts
            .drain()
            .flat_map(|cur| {
                // if i % 2 == 0 {
                // opts.insert(cur.clone());
                // }
                DIRS.iter()
                    .filter_map(|d| {
                        let next = &cur + &d;
                        let (y, x) = (
                            (next.0).rem_euclid(board.len() as isize) as usize,
                            (next.1).rem_euclid(board.len() as isize) as usize,
                        );
                        if next.0 < 0 - steps_over_bounds as isize
                            || next.0 > board.len() as isize + steps_over_bounds as isize - 1
                            || next.1 < 0 - steps_over_bounds as isize
                            || next.1 > board.len() as isize + steps_over_bounds as isize - 1
                            // || seen.contains(&next)
                            || board[y][x] == Cell::Rock
                        {
                            None
                        } else {
                            // seen.insert(next.clone());
                            Some(next.clone())
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            // .unique()
            .collect::<HashSet<Point>>();

        // if i % 2 == 1 {
        //     opts.extend(nextstarts.clone())
        // }
        // if nextstarts.len() == 0 {
        //     println!();
        //     println!(
        //         "max_y: {:?}, max_x: {:?}, max_c: {:?}",
        //         seen.iter().max_by_key(|Point(y, x)| y.abs()),
        //         seen.iter().max_by_key(|Point(y, x)| x.abs()),
        //         seen.iter().max_by_key(|Point(y, x)| x.abs() + y.abs())
        //     );
        //     return (opts.len(), i - offset - 1);
        // }
        swap(&mut starts, &mut nextstarts);
    }

    println!(
        "max_y: {:?}, max_x: {:?}",
        seen.iter().max_by_key(|Point(y, x)| y.abs()),
        seen.iter().max_by_key(|Point(y, x)| x.abs())
    );
    (starts.len(), 0)
    // (opts.len(), 0)
}

fn part2(lines: &Vec<String>, steps: usize) -> String {
    let (board, start) = parse(lines);

    let size = board.len();

    if steps <= 1000 {
        let (plots, _) =
            step_until_staturated_wrapped(&board, start.clone(), steps, 0, size * steps);
        if size == 11 {
            return format!("{}", plots);
        } else {
            println!("real: {}", plots);
        }
    }
    let mut seq = (0..4)
        .map(|i| {
            let (plots, s) =
                step_until_staturated_wrapped(&board, start.clone(), i * 131 + 65, 0, size * steps);
            // (i * 131 + 65, plots as isize)
            (plots as isize)
        })
        .collect_vec();
    println!("{:?}", seq);

    for _ in (seq.len() - 1..202300).progress() {
        let nxt = get_next_in_seq(&seq);
        seq.push(nxt as isize);
    }

    // WRONG: 613964166018543
    // WRONG: 606457395018193
    // WRONG: 609585229256082
    // WRONG: 609585229256083
    // WRONG: 609585229256085
    // WRONG: 609585229256086
    // WRONG: 609579202738978 -- sob
    // RIGHT: 609585229256084 -- so mad
    return format!("{}", seq.last().expect("must last"));

    // test some assumptions about the input
    assert_eq!(board.len(), board.first().expect("must first").len());
    assert_eq!(steps % size, size / 2);

    // calc the number of overlaps
    // calc saturations at different points
    println!(
        "steps:{}, size: {}, power: {}, rem: {}",
        steps,
        size,
        size.pow(2),
        steps % size
    );

    let boards = steps / size - 1;
    let (num_odd, num_even) = ((boards / 2 * 2 + 1).pow(2), ((boards + 1) / 2 * 2).pow(2));
    println!("boards: {}, o: {}, e: {}", boards, num_odd, num_even);

    let (odds, odd_steps) = step_until_staturated(&board, start.clone(), size * 2, 1);
    println!("odd: {}, s: {}", odds, odd_steps);

    let (evens, even_steps) = step_until_staturated(&board, start.clone(), size * 2, 0);
    println!("even: {}, s: {}", evens, even_steps);

    // odd and even plots should be occilating some how, figure out this formula
    /*
        |*|*|
        *|*|*
        |*|*|
        *|*|*
        |*|*|
    */
    let (x_evens, x_even_steps) =
        step_until_staturated_wrapped(&board, start.clone(), 196, 0, size * 5);
    println!("3x {}, s: {}", x_evens, x_even_steps,);
    let (x_odds, x_odd_steps) =
        step_until_staturated_wrapped(&board, start.clone(), 196, 1, size * 5);
    println!("3x {}, s: {}", x_odds, x_odd_steps,);

    let corner_offset = if num_even > num_odd { 1 } else { 0 };
    let (center_diamond, _) = step_until_staturated(&board, start.clone(), size / 2, corner_offset);
    let corners = if num_even > num_odd {
        odds - center_diamond
    } else {
        evens - center_diamond
    };
    dbg!(&corners);

    let edge_offset = if num_odd > num_even { 1 } else { 0 };

    let corners_2: usize = [
        Point(0, 0),
        Point(0, size as isize - 1),
        Point(size as isize - 1, size as isize - 1),
        Point(size as isize - 1, 0),
    ]
    .iter()
    .map(|p| step_until_staturated(&board, p.clone(), size / 2 - 1, corner_offset))
    .map(|(n, _)| dbg!(n))
    .sum();

    dbg!(&corners_2);

    let edges: usize = [
        Point(0, start.1),
        Point(start.0, size as isize - 1),
        Point(size as isize - 1, start.1),
        Point(start.0, 0),
    ]
    .iter()
    .map(|p| step_until_staturated(&board, p.clone(), size - 1, edge_offset))
    .map(|(n, _)| dbg!(n))
    .sum();
    // /^\  oeo
    // <#>  eoe
    // \v/  oeo
    dbg!(&edges);
    format!(
        "{}",
        (num_odd * odds) + (num_even * evens) + corners_2 + edges // + ((center_diamond) * (boards + 1))
    )
    // let o_down = (start.0 + steps).rem_euclid(board.len() as isize) as usize;
    // let o_up = (start.0 + steps).rem_euclid(board.len() as isize) as usize;
    // let o_right =
    //     (start.1 + steps).rem_euclid(board.first().expect("must first").len() as isize) as usize;
    // let o_left =
    //     (start.1 + steps).rem_euclid(board.first().expect("must first").len() as isize) as usize;

    // let d_down = (start.0 + steps).div_euclid(board.len() as isize) as usize;
    // let d_up = (start.0 + steps).div_euclid(board.len() as isize) as usize;
    // let d_right =
    //     (start.1 + steps).div_euclid(board.first().expect("must first").len() as isize) as usize;
    // let d_left =
    //     (start.1 + steps).div_euclid(board.first().expect("must first").len() as isize) as usize;

    // let saturated = board
    //     .iter()
    //     .flat_map(|row| row.iter())
    //     .filter(|&c| c == &Cell::Plot)
    //     .count();
    // let mut lens: HashSet<usize> = HashSet::new();
    // let mut starts: HashSet<Point> = HashSet::from([start.clone()]);
    // let mut seen: HashSet<Point> = HashSet::from([start.clone()]);
    // let mut seen_states: HashSet<String> = HashSet::new();
    // let mut seen_states_seen = 0;
    // for i in (0..steps).progress() {
    //     // let mut nextstarts = Vec::new();
    //     let mut nextstarts = starts
    //         .drain()
    //         .par_bridge()
    //         .flat_map(|cur| {
    //             DIRS.iter()
    //                 // .par_bridge()
    //                 .filter_map(|d| {
    //                     let next = &cur + &d;
    //                     let (y, x) = (
    //                         (next.0).rem_euclid(board.len() as isize) as usize,
    //                         (next.1).rem_euclid(board.first().expect("must first").len() as isize)
    //                             as usize,
    //                     );
    //                     if board[y][x] == Cell::Plot {
    //                         Some(next.clone())
    //                     } else {
    //                         None
    //                     }
    //                 })
    //                 .collect::<Vec<Point>>()
    //         })
    //         .collect::<HashSet<Point>>();
    //     let diff = nextstarts
    //         .difference(&seen)
    //         .map(|p| p.clone())
    //         .collect::<HashSet<Point>>();

    //     let k = diff
    //         .iter()
    //         .map(|p| {
    //             format!(
    //                 "{:?}",
    //                 Point((p.0).rem_euclid(max_y), (p.1).rem_euclid(max_x))
    //             )
    //         })
    //         .unique()
    //         .sorted()
    //         // .sorted_by_key(|&Point(y, x)| (!y, x))
    //         .collect_vec()
    //         .join(",");

    //     if seen_states.contains(&k) {
    //         println!(
    //             "{}: seen: {} next: {} diff: {}",
    //             i,
    //             seen.len(),
    //             nextstarts.len(),
    //             diff.len()
    //         );
    //         seen_states_seen += 1;
    //     }
    //     if seen_states_seen > 5 {
    //         // break;
    //     }
    //     // println!("{}", k);
    //     seen.extend(diff.clone());
    //     seen_states.insert(k);

    // if lens.contains(&nextstarts.len()) {
    //     println!("{}: seen: {} delta: {}", i, seen.len(), nextstarts.len());
    // }
    // if i > 1000 {
    //     lens.insert(nextstarts.len());
    // }

    // if nextstarts.len() % (((saturated + 1) / 2) + 1) / 10 == 0 {
    //     println!("{}: {}", i, nextstarts.len());
    // }
    //     swap(&mut starts, &mut nextstarts);
    // }

    // // let uniq_plots = (seen.len() + 1) / 2 + 1;
    // let uniq_plots = starts.len();
}

fn get_next_in_seq(seq: &Vec<isize>) -> usize {
    let mut deltavec: Vec<Vec<isize>> = Vec::new();
    deltavec.push(seq.clone());
    while !deltavec.last().expect("must last").iter().all(|&d| d == 0) {
        deltavec.push(
            deltavec
                .last()
                .expect("must elem")
                .iter()
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec()
                .clone(),
        );
    }
    // println!("{:?}", deltavec);
    deltavec
        .iter()
        .rev()
        .map(|d| d.last().expect("must elem"))
        .sum::<isize>() as usize
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines, 64));
    // println!("part 1 full: {:?}", part1(&lines, 7000));
    println!("part 2 - test: {:?}", part2(&lines, 196));
    // WRONG: 609581476388520 (too low)
    println!("part 2: {:?}", part2(&lines, 26501365));
}
