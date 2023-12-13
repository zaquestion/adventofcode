use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    println!("part 1: {:?}", part1(&lines));
    println!("part 2: {:?}", part2(&lines));
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Ash,
    Rock,
}

fn parse(lines: &Vec<String>) -> Vec<Vec<Vec<Cell>>> {
    lines
        .split(|l| l.is_empty())
        .map(|b| {
            b.iter()
                .map(|l| {
                    l.chars()
                        .map(|c| {
                            match c {
                                '.' => Some(Cell::Ash),
                                '#' => Some(Cell::Rock),
                                _ => None,
                            }
                            .expect("must resolve")
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> String {
    let boards = parse(lines);

    let sum = boards
        .iter()
        .map(|b| {
            if let Some(split) = find_reflection(b, |_| true) {
                Some((true, split))
            } else {
                let tb = transpose(b);
                if let Some(split) = find_reflection(&tb, |_| true) {
                    Some((false, split))
                } else {
                    None
                }
            }
        })
        .map(|s| s.expect("must resolve"))
        .fold(0, |acc, (h, v)| 
            //acc + (v + (h * 100))
            if h {
                acc + (v * 100)
            } else {
                acc + v
            }
        );

    format!("{}", sum)
}

fn part2(lines: &Vec<String>) -> String {
    let boards = parse(lines);

    let sum = boards
        .iter()
        .map(|b| {
            if let Some(split) = find_reflection(b, |_| true) {
                find_new_reflection(b, split, false)
            } else {
                let tb = transpose(b);
                if let Some(split) = find_reflection(&tb, |_| true) {
                    find_new_reflection(&tb, split, true)
                } else {
                    None
                }
            }
        })
        .map(|s| s.expect("must resolve"))
        .fold(0, |acc, (h, v)| 
            //acc + (v + (h * 100))
            if h {
                acc + (v * 100)
            } else {
                acc + v
            }
        );

    format!("{}", sum)
}

fn find_new_reflection(b: &Vec<Vec<Cell>>, split: usize, transposed: bool) -> Option<(bool,usize)> {
    (0..b.len()).cartesian_product(0..b[0].len()).map(|(y,x)| {
        let mut newb = b.clone();
        let c = newb[y][x];
        newb[y][x] = if c == Cell::Ash { Cell::Rock } else { Cell::Ash };
        newb
    }).skip_while(|newb| {
        let tb = transpose(&newb);
        {
            if let Some(newsplit) = find_reflection(&newb, |newsplit| &split != newsplit) {
                    Some((!transposed, newsplit))
            } else { 
                if let Some(newsplit) = find_reflection(&tb, |_| true) {
                    Some((transposed, newsplit))
                } else {
                    None
                }
            }
        }.is_none()
    }).filter_map(|newb| {
        let tb = transpose(&newb);
            if let Some(newsplit) = find_reflection(&newb, |newsplit| &split != newsplit) {
                    Some((!transposed, newsplit))
            } else { 
                if let Some(newsplit) = find_reflection(&tb, |_| true) {
                    Some((transposed, newsplit))
                } else {
                    None
                }
            }
    }).take(1).at_most_one().unwrap()
}

fn find_reflection(b: &Vec<Vec<Cell>>, f: impl Fn(&usize) -> bool) -> Option<usize> {
            b
                .iter()
                .enumerate()
                .tuple_windows()
                .filter(|(top, bottom)| top.1 == bottom.1)
                .filter_map(|split| {
                    if (0..split.0.0).rev()
                        .interleave_shortest(split.1.0+1..b.len())
                        .tuples()
                        .all(|(u, d)| {
                              // dbg!(u) == dbg!(d);
                              &b[u] == &b[d]
                            }
                        )
                    {
                        Some(split.1 .0)
                    } else {
                        None
                    }
                }).filter(f)
        .at_most_one().unwrap()
}

fn render_board(b: &Vec<Vec<Cell>>, transposed: bool) {
    println!();
    println!("transposed: {}", transposed);
    b.iter().for_each(|l| {
        println!(
            "{}",
            l.iter()
                .map(|c| match c {
                    Cell::Ash => '.',
                    Cell::Rock => '#',
                })
                .collect::<String>()
        );
    });
}

fn transpose(b: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
        let mut col_iters = b.iter().map(|l| l.into_iter()).collect_vec();
        (0..b[0].len())
            .map(|_| {
                col_iters
                    .iter_mut()
                    .map(|iter| *iter.next().expect("must iter"))
                    .collect_vec()
            }) .collect_vec()
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn sampledata() -> Vec<String> {
        vec![
            "#.##..##.".to_string(),
            "..#.##.#.".to_string(),
            "##......#".to_string(),
            "##......#".to_string(),
            "..#.##.#.".to_string(),
            "..##..##.".to_string(),
            "#.#.##.#.".to_string(),
            "".to_string(),
            "#...##..#".to_string(),
            "#....#..#".to_string(),
            "..##..###".to_string(),
            "#####.##.".to_string(),
            "#####.##.".to_string(),
            "..##..###".to_string(),
            "#....#..#".to_string(),
        ]
    }

    #[rstest]
    #[case(sampledata(), "405")]
    fn test_part1_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part1(&input));
    }

    #[rstest]
    #[case(sampledata(), "400")]
    fn test_part2_sample(#[case] input: Vec<String>, #[case] expected: String) {
        assert_eq!(expected, part2(&input));
    }
}
