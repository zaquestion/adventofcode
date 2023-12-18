use itertools::Itertools;
use std::io;
use std::ops::Add;
use std::ops::AddAssign;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1_fast(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
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

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        };
    }
}

fn _part1(lines: &Vec<String>) -> String {
    let dig_plans = parse(lines);

    let mut cur = Point(0, 0);
    let mut plots: Vec<Point> = vec![];
    for (dir, num_plots, _) in dig_plans.iter() {
        for _ in 0..*num_plots {
            cur += *dir;
            plots.push(cur.clone());
        }
    }

    format!("{}", _calc_area(&mut plots) + plots.len())
}

fn part1_fast(lines: &Vec<String>) -> String {
    let dig_plans = parse(lines);

    let mut cur = Point(0, 0);
    let mut outside_plots = 0;
    let mut corners: Vec<Point> = vec![];
    for (dir, num_plots, _) in dig_plans.iter() {
        outside_plots += num_plots;
        for _ in 0..*num_plots {
            cur += *dir;
        }
        corners.push(cur.clone());
    }

    format!("{}", calc_area2(&corners) + (outside_plots / 2) + 1)
}

fn part2(lines: &Vec<String>) -> String {
    let dig_plans = parse(lines);

    let mut cur = Point(0, 0);
    let mut outside_plots = 0;
    let mut corners: Vec<Point> = vec![];
    for (_, _, hex) in dig_plans.iter() {
        let (dir, num_plots) = hex_to_plan(hex);
        outside_plots += num_plots;
        for _ in 0..num_plots {
            cur += dir;
            corners.push(cur.clone());
        }
    }

    format!("{}", calc_area2(&corners) + (outside_plots / 2) + 1)
}

const DIRS: [Point; 4] = [Point(0, 1), Point(1, 0), Point(0, -1), Point(-1, 0)];
fn hex_to_plan(hex: &String) -> (Point, usize) {
    (
        DIRS[hex
            .chars()
            .last()
            .expect("must elem")
            .to_string()
            .parse::<usize>()
            .expect("must num")],
        usize::from_str_radix(&hex[..5], 16).expect("must decimal"),
    )
}

fn _calc_area(points: &mut Vec<Point>) -> usize {
    let min_x = points.iter().map(|p| p.1).min().unwrap();
    let max_x = points.iter().map(|p| p.1).max().unwrap();
    let min_y = points.iter().map(|p| p.0).min().unwrap();
    let max_y = points.iter().map(|p| p.0).max().unwrap();

    (min_y..max_y)
        .cartesian_product(min_x..max_x)
        .fold(0, |acc, (y, x)| {
            if _is_interior_point(&points, &Point(y, x)) {
                acc + 1
            } else {
                acc
            }
        })
}

fn calc_area2(corners: &Vec<Point>) -> usize {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..corners.len() {
        let next_index = (i + 1) % corners.len();
        sum1 += corners[i].0 * corners[next_index].1;
        sum2 += corners[i].1 * corners[next_index].0;
    }

    ((sum1 - sum2).abs() / 2) as usize
}

fn _is_interior_point(loop_nodes: &Vec<Point>, point: &Point) -> bool {
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

fn parse(lines: &Vec<String>) -> Vec<(Point, usize, String)> {
    lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .tuples()
                .map(|(d, n, col)| {
                    (
                        match d {
                            "R" => Some(Point(0, 1)),
                            "L" => Some(Point(0, -1)),
                            "U" => Some(Point(-1, 0)),
                            "D" => Some(Point(1, 0)),
                            _ => None,
                        }
                        .expect("must resolve"),
                        n.parse::<usize>().expect("must num"),
                        col.strip_prefix("(#")
                            .expect("must (#")
                            .strip_suffix(")")
                            .expect("must )")
                            .to_string(),
                    )
                })
                .exactly_one()
                .expect("must one")
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "R 6 (#70c710)".to_string(),
            "D 5 (#0dc571)".to_string(),
            "L 2 (#5713f0)".to_string(),
            "D 2 (#d2c081)".to_string(),
            "R 2 (#59c680)".to_string(),
            "D 2 (#411b91)".to_string(),
            "L 5 (#8ceee2)".to_string(),
            "U 2 (#caa173)".to_string(),
            "L 1 (#1b58a2)".to_string(),
            "U 2 (#caa171)".to_string(),
            "R 2 (#7807d2)".to_string(),
            "U 3 (#a77fa3)".to_string(),
            "L 2 (#015232)".to_string(),
            "U 2 (#7a21e3)".to_string(),
        ]
    }

    fn simple_box_data() -> Vec<String> {
        vec![
            "R 6 (#000000)".to_string(),
            "D 2 (#000000)".to_string(),
            "L 6 (#000000)".to_string(),
            "U 2 (#000000)".to_string(),
        ]
    }

    fn simple_l_shape_data() -> Vec<String> {
        vec![
            "R 6 (#000000)".to_string(),
            "D 4 (#000000)".to_string(),
            "L 3 (#000000)".to_string(),
            "U 2 (#000000)".to_string(),
            "L 3 (#000000)".to_string(),
            "U 2 (#000000)".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "62")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1_fast(&input));
    }

    #[rstest]
    #[case(sampledata(), "62")]
    #[case(simple_box_data(), "21")]
    #[case(simple_l_shape_data(), "29")]
    fn test_part1_fast(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1_fast(&input));
    }

    #[rstest]
    #[case("70c710", (Point(0,1), 461937))]
    fn test_hex_to_plan(#[case] input: String, #[case] expected: (Point, usize)) {
        assert_eq!(expected, hex_to_plan(&input));
    }

    #[rstest]
    #[case(vec![Point(0, 0),
                Point(0, 6),
                Point(2, 6),
                Point(2, 0),
    ], 12)]
    #[case(vec![Point(0, 0),
                Point(0, 7),
                Point(3, 7),
                Point(3, 0),
    ], 21)]
    fn test_calc_area2(#[case] input: Vec<Point>, #[case] expected: usize) {
        assert_eq!(expected, calc_area2(&input));
    }

    #[rstest]
    #[case(sampledata(), "952408144115")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}
