use std::io;
use std::collections::HashSet;

fn part1(lines: &Vec<String>) {
    let line = &lines[0];
    let uw = 4;
    for i in 0..line.len() {
        if line[i..i+uw].chars().collect::<HashSet<char>>().len() == uw {
            println!("{}", i+uw);
            return
        }
    }
    println!("invalid")
}

fn part2(lines: &Vec<String>) {
    let line = &lines[0];
    let uw = 14;
    for i in 0..line.len() {
        if line[i..i+uw].chars().collect::<HashSet<char>>().len() == uw {
            println!("{}", i+uw);
            return
        }
    }
    println!("invalid")
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    part1(&lines);
    part2(&lines);
}
