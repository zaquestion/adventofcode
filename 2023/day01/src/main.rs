use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let sum: isize = lines
        .iter()
        .map(|l| -> isize {
            let left: char = l
                .chars()
                .skip_while(|c| !c.is_numeric())
                .take(1)
                .exactly_one()
                .expect("must once");
            let right: char = l
                .chars()
                .rev()
                .skip_while(|c| !c.is_numeric())
                .take(1)
                .exactly_one()
                .expect("must once");
            // .to_string()
            // .parse::<isize>()
            // .expect("must num");
            let n: String = left.to_string() + &right.to_string();
            n.parse::<isize>().expect("num")
        })
        .sum();
    println!("part1: {}", sum);

    let sum: isize = lines
        .iter()
        .map(|l| -> isize {
            let left: char = l
                .chars()
                .enumerate()
                .filter_map(|(i, c)| {
                    if c.is_numeric() {
                        return Some(c);
                    }
                    if l.len() - i < 3 {
                        return None;
                    }
                    let d = match dbg!(&l[i..i + 3]) {
                        "one" => Some('1'),
                        "two" => Some('2'),
                        "six" => Some('6'),
                        _ => None,
                    };
                    if d.is_some() {
                        return d;
                    }
                    if l.len() - i < 4 {
                        return None;
                    }
                    let d = match dbg!(&l[i..i + 4]) {
                        "four" => Some('4'),
                        "five" => Some('5'),
                        "nine" => Some('9'),
                        _ => None,
                    };
                    if d.is_some() {
                        return d;
                    }
                    if l.len() - i < 5 {
                        return None;
                    }
                    let d = match dbg!(&l[i..i + 5]) {
                        "three" => Some('3'),
                        "seven" => Some('7'),
                        "eight" => Some('8'),
                        _ => None,
                    };
                    d
                })
                .take(1)
                .exactly_one()
                .expect("must once");

            let right: char = l
                .chars()
                .rev()
                .enumerate()
                .filter_map(|(i, c)| {
                    let i = l.len() - i;
                    dbg!(i);
                    if c.is_numeric() {
                        return Some(c);
                    }
                    if i as isize - 3 < 0 {
                        return None;
                    }
                    let d = match dbg!(&l[i - 3..i]) {
                        "one" => Some('1'),
                        "two" => Some('2'),
                        "six" => Some('6'),
                        _ => None,
                    };
                    if d.is_some() {
                        return d;
                    }
                    if i as isize - 4 < 0 {
                        return None;
                    }
                    let d = match dbg!(&l[i - 4..i]) {
                        "four" => Some('4'),
                        "five" => Some('5'),
                        "nine" => Some('9'),
                        _ => None,
                    };
                    if d.is_some() {
                        return d;
                    }
                    if i as isize - 5 < 0 {
                        return None;
                    }
                    let d = match dbg!(&l[i - 5..i]) {
                        "three" => Some('3'),
                        "seven" => Some('7'),
                        "eight" => Some('8'),
                        _ => None,
                    };
                    d
                })
                .take(1)
                .exactly_one()
                .expect("must once");

            let n: String = left.to_string() + &right.to_string();
            n.parse::<isize>().expect("num")
        })
        .sum();
    println!("part2: {}", sum);
}
