use std::io;

fn part1(lines: &Vec<String>) {
    let mut nums: Vec<isize> = Vec::new();
    for line in lines {
        nums.push(line.parse::<isize>().expect("must isize"));
    }

    let mut out = (0..nums.len()).collect::<Vec<_>>();

    for (i, o) in nums.iter().enumerate() {
        let idx = out.iter().position(|&n| n == i).unwrap();
        out.remove(idx);
        let next_i = (idx as isize + o).rem_euclid(out.len() as isize);
        out.insert(next_i as usize, i);
    }

    let orig_i = nums.iter().position(|&i| i == 0).unwrap();
    let i = out.iter().position(|&i| i == orig_i).unwrap();
    let a = nums[out[(i + 1000) % out.len()]];
    let b = nums[out[(i + 2000) % out.len()]];
    let c = nums[out[(i + 3000) % out.len()]];
    println!("part 1: {}", a + b + c);
    // WRONG: 1926
    // WRONG: -6656
    // WRONG: 9544
    // WRONG: 15077
    // WRONG: 17994
    // WRONG: -2585
    // WRONG: -4915
    // WRONG: 13642
    // RIGHT: 1591
}

fn part2(lines: &Vec<String>) {
    let mut nums: Vec<isize> = Vec::new();
    for line in lines {
        nums.push(line.parse::<isize>().expect("must isize"));
    }

    let mut out = (0..nums.len()).collect::<Vec<_>>();
    nums = nums.iter().map(|n| n * 811589153).collect::<Vec<isize>>();

    for _ in 0..10 {
        for (i, o) in nums.iter().enumerate() {
            let idx = out.iter().position(|&n| n == i).unwrap();
            out.remove(idx);
            let next_i = (idx as isize + o).rem_euclid(out.len() as isize);
            out.insert(next_i as usize, i);
        }
    }

    let orig_i = nums.iter().position(|&i| i == 0).unwrap();
    let i = out.iter().position(|&i| i == orig_i).unwrap();
    let a = nums[out[(i + 1000) % out.len()]];
    let b = nums[out[(i + 2000) % out.len()]];
    let c = nums[out[(i + 3000) % out.len()]];
    println!("part 2: {}", a + b + c);
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    part1(&lines);
    part2(&lines);
}
