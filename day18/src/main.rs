use std::collections::{HashMap, HashSet};
use std::io;

fn part1(lines: &Vec<String>) {
    let mut blocks: HashSet<(isize, isize, isize)> = HashSet::new();
    for line in lines.iter() {
        let nums = line
            .trim()
            .split(',')
            .map(|c| c.parse::<isize>().expect("must num"))
            .collect::<Vec<_>>();
        blocks.insert((nums[0], nums[1], nums[2]));
    }

    let mut surfaces: Vec<(isize, isize, isize)> = Vec::new();
    for block in blocks.iter() {
        let mut sides = faces(*block, 1);
        surfaces.append(&mut sides);
    }
    let all_sides = surfaces.clone();

    surfaces.retain(|b| !blocks.contains(b));

    println!("part1: {:?}", surfaces.len());

    let enclosed = enclosed(&blocks, &all_sides);
    surfaces.retain(|s| !enclosed.contains(s));

    //println!("part2: {:?}", surfaces.len());
    println!("part2: {:?}", surfaces.len());
}

fn enclosed(
    blocks: &HashSet<(isize, isize, isize)>,
    surfaces: &Vec<(isize, isize, isize)>,
) -> HashSet<(isize, isize, isize)> {
    let mut enclosed: HashSet<(isize, isize, isize)> = HashSet::new();

    let mut maxmax = isize::MIN;
    blocks.iter().for_each(|b| {
        if b.0 > maxmax {
            maxmax = b.0;
        }
        if b.1 > maxmax {
            maxmax = b.1;
        }
        if b.2 > maxmax {
            maxmax = b.1;
        }
    });
    maxmax += 1;

    // a bit surprised this works... my input must be forgiving as I'm pretty sure this wouldn't
    // account for small openings that lead out to the surface in particular caves, I was expecting
    // to have to filter after this step...
    surfaces.iter().for_each(|s| {
        let shoosts = shoost(*s, maxmax);
        let is_enclosed = vec![
            shoosts[0].iter().any(|s| blocks.contains(s)),
            shoosts[1].iter().any(|s| blocks.contains(s)),
            shoosts[2].iter().any(|s| blocks.contains(s)),
            shoosts[3].iter().any(|s| blocks.contains(s)),
            shoosts[4].iter().any(|s| blocks.contains(s)),
            shoosts[5].iter().any(|s| blocks.contains(s)),
        ]
        .iter()
        .all(|shot| *shot);
        if is_enclosed {
            enclosed.insert(*s);
        }
    });

    enclosed
}

fn faces(cube: (isize, isize, isize), i: isize) -> Vec<(isize, isize, isize)> {
    let mut sides: Vec<(isize, isize, isize)> = Vec::new();

    sides.push((cube.0 - i, cube.1, cube.2));
    sides.push((cube.0, cube.1 - i, cube.2));
    sides.push((cube.0, cube.1, cube.2 - i));

    sides.push((cube.0 + i, cube.1, cube.2));
    sides.push((cube.0, cube.1 + i, cube.2));
    sides.push((cube.0, cube.1, cube.2 + i));

    sides
}

fn shoost(cube: (isize, isize, isize), i: isize) -> Vec<Vec<(isize, isize, isize)>> {
    let mut shoosts: Vec<Vec<(isize, isize, isize)>> = Vec::new();

    let mut sides: Vec<(isize, isize, isize)> = Vec::new();
    for ii in 0..i {
        sides.push((cube.0 - ii, cube.1, cube.2));
    }
    shoosts.push(sides);

    let mut sides: Vec<(isize, isize, isize)> = Vec::new();
    for ii in 0..i {
        sides.push((cube.0, cube.1 - ii, cube.2));
    }
    shoosts.push(sides);

    let mut sides: Vec<(isize, isize, isize)> = Vec::new();
    for ii in 0..i {
        sides.push((cube.0, cube.1, cube.2 - ii));
    }
    shoosts.push(sides);

    let mut sides: Vec<(isize, isize, isize)> = Vec::new();
    for ii in 0..i {
        sides.push((cube.0 + ii, cube.1, cube.2));
    }
    shoosts.push(sides);

    let mut sides: Vec<(isize, isize, isize)> = Vec::new();
    for ii in 0..i {
        sides.push((cube.0, cube.1 + ii, cube.2));
    }
    shoosts.push(sides);

    let mut sides: Vec<(isize, isize, isize)> = Vec::new();
    for ii in 0..i {
        sides.push((cube.0, cube.1, cube.2 + ii));
    }
    shoosts.push(sides);

    shoosts
}

fn part2(_lines: &Vec<String>) {}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    part1(&lines);
    part2(&lines);
}
