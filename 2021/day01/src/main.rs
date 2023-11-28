use itertools::Itertools;
use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    let nums = lines
        .iter()
        .map(|l| l.parse::<isize>().expect("must num"))
        .collect::<Vec<isize>>();

    let increases = nums.iter().tuple_windows::<(&isize, &isize)>().fold(
        0,
        |acc: isize, x: (&isize, &isize)| if x.0 < x.1 { acc + 1 } else { acc },
    );

    println!("part1: {}", increases);

    let batch_increases = nums
        .windows(3)
        .map(|v| v.iter().sum())
        .tuple_windows::<(isize, isize)>()
        .fold(
            0,
            |acc: isize, x: (isize, isize)| if x.0 < x.1 { acc + 1 } else { acc },
        );

    println!("part2: {}", batch_increases);
}
