use itertools::Itertools;
use std::io;
use std::ops::Add;
use z3::ast::{Ast, Int};

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    // px py pz @ vx vy vz
    fn sampledata() -> Vec<String> {
        vec![
            "19, 13, 30 @ -2,  1, -2".to_string(),
            "18, 19, 22 @ -1, -1, -2".to_string(),
            "20, 25, 34 @ -2, -2, -4".to_string(),
            "12, 31, 28 @ -1, -2, -1".to_string(),
            "20, 19, 15 @  1, -5, -3".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), 7.0..=27.0, "2")]
    fn test_part1_sample(
        #[case] input: Vec<String>,
        #[case] range: std::ops::RangeInclusive<f64>,
        #[case] expected: String,
    ) {
        assert_eq!(expected, part1(&input, range));
    }

    #[rstest]
    #[case(sampledata(), "47")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn parse(lines: &Vec<String>) -> Vec<(Point, Point)> {
    lines
        .iter()
        .map(|l| {
            l.split(" @ ")
                .map(|p| {
                    p.split(", ")
                        .map(|n| n.trim().parse::<f64>().expect("must num"))
                        .tuples()
                        .map(|(x, y, z)| Point { x, y, z })
                        .exactly_one()
                        .expect("must one")
                })
                .collect_tuple::<(Point, Point)>()
                .expect("must tuple")
        })
        .collect_vec()
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn intersection_2d(&self, line2: &Line) -> Option<Point> {
        let dx1 = self.end.x - self.start.x;
        let dy1 = self.end.y - self.start.y;
        let dx2 = line2.end.x - line2.start.x;
        let dy2 = line2.end.y - line2.start.y;

        let m1 = dy1 / dx1;
        let m2 = dy2 / dx2;

        if (m2 - m1).abs() < f64::EPSILON {
            return None;
        }

        let x = (m1 * self.start.x - m2 * line2.start.x + line2.start.y - self.start.y) / (m1 - m2);
        let y = (m1 * m2 * (line2.start.x - self.start.x) + m2 * self.start.y - m1 * line2.start.y)
            / (m2 - m1);

        Some(Point { x, y, z: 0. })
    }
    fn intersects_2d(&self, other: &Line) -> bool {
        points_counterclockwise(&self.start, &other.start, &other.end)
            != points_counterclockwise(&self.end, &other.start, &other.end)
            && points_counterclockwise(&self.start, &self.end, &other.start)
                != points_counterclockwise(&self.start, &self.end, &other.end)
    }
    fn intersects_rect(&self, rect: (Point, Point)) -> bool {
        let (a, b) = rect;
        let rect_edges = [
            Line {
                start: Point {
                    x: a.x,
                    y: a.y,
                    z: 0.,
                },
                end: Point {
                    x: b.x,
                    y: a.y,
                    z: 0.,
                },
            }, // Bottom edge
            Line {
                start: Point {
                    x: b.x,
                    y: a.y,
                    z: 0.,
                },
                end: Point {
                    x: b.x,
                    y: b.y,
                    z: 0.,
                },
            }, // Right edge
            Line {
                start: Point {
                    x: b.x,
                    y: b.y,
                    z: 0.,
                },
                end: Point {
                    x: a.x,
                    y: b.y,
                    z: 0.,
                },
            }, // Top edge
            Line {
                start: Point {
                    x: a.x,
                    y: b.y,
                    z: 0.,
                },
                end: Point {
                    x: a.x,
                    y: a.y,
                    z: 0.,
                },
            }, // Left edge
        ];

        rect_edges.iter().any(|edge| self.intersects_2d(edge))
    }
}

fn points_counterclockwise(a: &Point, b: &Point, c: &Point) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}

fn part1(lines: &Vec<String>, range: std::ops::RangeInclusive<f64>) -> String {
    let hail = parse(lines);

    let intersect_count = hail
        .iter()
        .map(|(p, v)| Line {
            start: p.clone(),
            end: p + v,
        })
        .tuple_combinations()
        .filter_map(|(a, b)| {
            if let Some(Point { x, y, z }) = a.intersection_2d(&b) {
                let dx1 = a.end.x - a.start.x;
                let dx2 = b.end.x - b.start.x;
                if dx1.signum() != (x - a.start.x).signum()
                    || dx2.signum() != (x - b.start.x).signum()
                {
                    None
                } else if range.contains(&x) && range.contains(&y) {
                    Some(Point { x, y, z: 0. })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .inspect(|p| {
            // dbg!(p);
        })
        .count();

    format!("{}", intersect_count)
}

fn part2(lines: &Vec<String>) -> String {
    let hail = parse(lines);
    // https://siedentop.dev/posts/rust-z3/

    let ctx = z3::Context::new(&z3::Config::new());
    let s = z3::Solver::new(&ctx);
    let [fx, fy, fz, fdx, fdy, fdz] =
        ["fx", "fy", "fz", "fdx", "fdy", "fdz"].map(|v| Int::new_const(&ctx, v));

    let zero = Int::from_i64(&ctx, 0);
    for (i, (p, v)) in hail.iter().enumerate() {
        let (
            Point { x, y, z },
            Point {
                x: dx,
                y: dy,
                z: dz,
            },
        ) = (p, v);
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, *v as i64));
        let t = Int::new_const(&ctx, format!("t{i}"));
        s.assert(&t.ge(&zero));
        s.assert(&((&x + &dx * &t)._eq(&(&fx + &fdx * &t))));
        s.assert(&((&y + &dy * &t)._eq(&(&fy + &fdy * &t))));
        s.assert(&((&z + &dz * &t)._eq(&(&fz + &fdz * &t))));
    }
    assert_eq!(s.check(), z3::SatResult::Sat);
    let model = s.get_model().unwrap();
    let res = model.eval(&(&fx + &fy + &fz), true).expect("must eval");

    format!("{}", res.as_i64().expect("must i64"))
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    // WRONG: 11353 (low)
    println!(
        "part 1: {:?}",
        part1(&lines, 200_000_000_000_000.0..=400_000_000_000_000.0)
    );
    // WRONG: 415263125472448 (low)
    println!("part 2: {:?}", part2(&lines));
}
