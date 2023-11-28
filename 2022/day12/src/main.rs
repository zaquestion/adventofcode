use std::io;
use pathfinding::prelude::astar;
use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(char, usize, usize);

fn successors(lines: &Vec<String>, p: &Pos) -> Vec<(Pos, i32)> {
    let grid = lines.clone();
    let mut ss: Vec<(Pos, i32)> = Vec::new();
    let cur = p.0;
    let row = p.1;
    let col = p.2;

    if col > 0 {
        let l = grid[row].chars().nth(col-1).unwrap();
        let mut cost = l as i32 - cur as i32;
        if l == 'E' {
            cost = 'z' as i32 - cur as i32;
        }
        if cost < 1 {
            cost = 1
        }
        if l != 'S' && cost <= 1 {
            ss.push((Pos(l, row, col-1), cost));
        }
    }
    if col < grid[row].len() - 1 {
        let r = grid[row].chars().nth(col+1).unwrap();
        let mut cost = r as i32 - cur as i32;
        //println!("r -- {:?} -- {:?}", r, cost);
        if r == 'E' {
            cost = 'z' as i32 - cur as i32;
        }
        if cost < 1 {
            cost = 1
        }
        if r != 'S' && cost <= 1 {
            ss.push((Pos(r, row, col+1), cost));
        }
    }
    if row > 0 {
        let u = grid[row - 1].chars().nth(col).unwrap();
        let mut cost = u as i32 - cur as i32;
        if u == 'E' {
            cost = 'z' as i32 - cur as i32;
        }
        if cost < 1 {
            cost = 1
        }
        if u != 'S' && cost <= 1 {
            ss.push((Pos(u, row-1, col), cost));
        }
    }
    if row < grid.len() - 1 {
        let d = grid[row + 1].chars().nth(col).unwrap();
        let mut cost = d as i32 - cur as i32;
        if d == 'E' {
            cost = 'z' as i32 - cur as i32;
        }
        if cost < 1 {
            cost = 1
        }
        if d != 'S' && cost <= 1 {
            ss.push((Pos(d, row+1, col), cost));
        }
    }

    ss
}

fn distance(a: &Pos, b: &Pos) -> u32 {
    (a.1.abs_diff(b.1) + a.2.abs_diff(b.2)) as u32
}

fn successors_bfs(lines: &Vec<String>, p: &Pos) -> Vec<Pos> {
    let grid = lines.clone();
    let mut ss: Vec<Pos> = Vec::new();
    let cur = p.0;
    let row = p.1;
    let col = p.2;

    if col > 0 {
        let l = grid[row].chars().nth(col-1).unwrap();
        let mut cost = l as i32 - cur as i32;
        if l == 'E' {
            cost = 'z' as i32 - cur as i32;
        }
        if l != 'S' && cost <= 1 {
            ss.push(Pos(l, row, col-1));
        }
    }
    if col < grid[row].len() - 1 {
        let r = grid[row].chars().nth(col+1).unwrap();
        let mut cost = r as i32 - cur as i32;
        //println!("r -- {:?} -- {:?}", r, cost);
        if r == 'E' {
            cost = 'z' as i32 - cur as i32;
        }
        if r != 'S' && cost <= 1 {
            ss.push(Pos(r, row, col+1));
        }
    }
    if row > 0 {
        let u = grid[row - 1].chars().nth(col).unwrap();
        let mut cost = u as i32 - cur as i32;
        if u == 'E' {
            cost = 'z' as i32 - cur as i32;
        }
        if u != 'S' && cost <= 1 {
            ss.push(Pos(u, row-1, col));
        }
    }
    if row < grid.len() - 1 {
        let d = grid[row + 1].chars().nth(col).unwrap();
        let mut cost = d as i32 - cur as i32;
        if d == 'E' {
            cost = 'z' as i32 - cur as i32;
        }
        if d != 'S' && cost <= 1 {
            ss.push(Pos(d, row+1, col));
        }
    }

    //println!("{:?} -- {:?}", p,  ss);

    ss
}



fn part1_and_2(lines: &Vec<String>) {
    let mut s: Pos = Pos('S', 0, 0);
    let mut e: Pos = Pos('E', 0, 0);
    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == 'S' {
                s = Pos('a', i, j);
            }
            if c == 'E' {
                println!("Coordinates of 'E': ({}, {})", i, j);
                e = Pos('E', i, j);
            }
        }
    }

    let resulta = astar(&s, |p| successors(lines, p), |p| distance(p, &e).try_into().unwrap(),
                       |p| p.0 == e.0).unwrap();


    println!("A Steps: {:?}", resulta.0.len()-1);

    let resultb = bfs(&s, |p| successors_bfs(lines, p),
                       |p| p.0 == e.0).unwrap();
    println!("Steps: {:?}", resultb.len()-1);

    let mut minmin = 99999999;
    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == 'a' {
                s = Pos('a', i, j);
                let resulta = astar(&s, |p| successors(lines, p), |p| distance(p, &e).try_into().unwrap(),
                       |p| p.0 == e.0).unwrap_or_default();
                //let resultb = bfs(&s, |p| successors_bfs(lines, p), |p| p.0 == e.0).unwrap_or_default();
                if resulta.0.len() > 0 && resulta.0.len()-1 < minmin  {
                    minmin = resulta.0.len() - 1;
                }
            }
        }
    }
    println!("Min Min: {:?}", minmin);

    /*
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi

        v..v<<<<
        >v.vv<<^
        .>vv>E^^
        ..v>>>^^
        ..>>>>>^
    */
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    //let input = io::read_to_string(io::stdin()).unwrap();

    part1_and_2(&lines);
}
