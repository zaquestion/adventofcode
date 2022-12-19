use crossbeam_channel::{bounded, select};
use std::io;

#[derive(Clone, PartialEq)]
enum Cell {
    Open,
    Sensor,
    Beacon,
    InRange,
}

fn parse(lines: &Vec<String>) -> (Vec<(isize, isize)>, Vec<(isize, isize)>, isize, isize) {
    let mut sensors: Vec<(isize, isize)> = Vec::new();
    let mut beacons: Vec<(isize, isize)> = Vec::new();
    for line in lines.iter() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let sx = parts[2]
            .trim_start_matches("x=")
            .trim_end_matches(",")
            .parse::<isize>()
            .expect("must num");
        let sy = parts[3]
            .trim_start_matches("y=")
            .trim_end_matches(":")
            .parse::<isize>()
            .expect("must num");
        sensors.push((sx, sy));

        let bx = parts[8]
            .trim_start_matches("x=")
            .trim_end_matches(",")
            .parse::<isize>()
            .expect("must num");
        let by = parts[9]
            .trim_start_matches("y=")
            .parse::<isize>()
            .expect("must num");
        beacons.push((bx, by));
    }

    let mut maxx: isize;
    let mut maxy: isize;
    let mut minx: isize;
    let mut miny: isize;

    let smx = sensors.iter().map(|s| s.0).max().unwrap();
    let bmx = beacons.iter().map(|b| b.0).max().unwrap();
    maxx = smx;
    if bmx > maxx {
        maxx = bmx;
    }

    let smx = sensors.iter().map(|s| s.0).min().unwrap();
    let bmx = beacons.iter().map(|b| b.0).min().unwrap();
    minx = smx;
    if bmx < minx {
        minx = bmx;
    }
    (sensors, beacons, minx, maxx)
}

fn calc_row(
    sensors: &Vec<(isize, isize)>,
    beacons: &Vec<(isize, isize)>,
    row: isize,
    minx: isize,
    maxx: isize,
    maxd: isize,
) -> Vec<Cell> {
    let size = ((maxx - minx) * 2) as usize;
    let mut field: Vec<Cell> = vec![Cell::Open; size];
    let xoff = maxd;
    let y = row;
    for i in 0..sensors.len() {
        let s = sensors[i];
        let b = beacons[i];
        let d = dist(s, b);

        // Begin pt 2 optimizations
        let rd = dist(s, (s.0, row));
        if rd > d + 1 {
            continue;
        }

        let sd = d - rd;
        // End pt 2 optimizations

        let mut x = s.0 - sd - 1;
        loop {
            if x > s.0 + sd + 1 {
                break;
            }
            x += 1;
            let (ux, _) = cords(x as isize, y, xoff, 0);
            if ux < 0 || ux > field.len() as isize - 1 {
                continue;
            }
            if field[ux as usize] != Cell::Open {
                continue;
            }
            if dist(s, (x as isize, y)) <= d {
                field[ux as usize] = Cell::InRange;
            }
        }
    }
    for (x, y) in sensors.iter() {
        if *y != row {
            continue;
        }
        let (ux, _) = cords(*x, *y, xoff, 0);
        if ux < 0 || ux > field.len() as isize - 1 {
            continue;
        }
        field[ux as usize] = Cell::Sensor;
    }
    for (x, y) in beacons.iter() {
        if *y != row {
            continue;
        }
        let (ux, _) = cords(*x, *y, xoff, 0);
        if ux < 0 || ux > field.len() as isize - 1 {
            continue;
        }
        field[ux as usize] = Cell::Beacon;
    }
    return field;
}

fn show_row(row: &Vec<Cell>) {
    for c in row.iter() {
        match c {
            Cell::Open => print!("."),
            Cell::Sensor => print!("S"),
            Cell::Beacon => print!("B"),
            Cell::InRange => print!("#"),
        }
    }
    println!();
}

fn cords(x: isize, y: isize, xoff: isize, yoff: isize) -> (isize, isize) {
    ((x + xoff), (y + yoff))
}

fn dist(s: (isize, isize), b: (isize, isize)) -> isize {
    (s.0.abs_diff(b.0) + s.1.abs_diff(b.1)) as isize
    //isize::abs(s.0 - b.0) + isize::abs(s.1 - b.1)
}

fn part1(lines: &Vec<String>, y: isize) {
    let (sensors, beacons, minx, maxx) = parse(lines);

    let mut maxd = isize::MIN;
    for i in 0..sensors.len() {
        let s = sensors[i];
        let b = beacons[i];
        let d = dist(s, b);
        if d > maxd {
            maxd = d;
        }
    }

    let row = calc_row(&sensors, &beacons, y, minx, maxx, maxd);
    if y == 10 {
        show_row(&row);
    }

    println!(
        "part1: y={}: {:?}",
        y,
        row.iter().filter(|c| (**c == Cell::InRange)).count()
    );
}

fn tuning_freq(b: (usize, usize)) -> usize {
    ((b.0 * 4_000_000) + b.1) as usize
}

fn part2(lines: &Vec<String>, max: usize) {
    let (sensors, beacons, _minx, _maxx) = parse(lines);
    let mut maxd = isize::MIN;
    for i in 0..sensors.len() {
        let s = sensors[i];
        let b = beacons[i];
        let d = dist(s, b);
        if d > maxd {
            maxd = d;
        }
    }
    println!("maxd: {}", maxd);

    let tp = threadpool::ThreadPool::new(96);
    let (tx, rx) = bounded::<usize>(0);

    for y in (0..max).rev() {
        let tx = tx.clone();
        let ss = sensors.clone();
        let bb = beacons.clone();
        tp.execute(move || {
            let row = calc_row(&ss, &bb, y as isize, 0, max as isize - maxd / 2 + 1, maxd);
            for x in maxd as usize..max + maxd as usize {
                if row[x] == Cell::Open {
                    let rx = x - maxd as usize;
                    tx.send(tuning_freq((rx, y))).expect("no receiver");
                    return;
                }
            }
        });
    }
    loop {
        select! {
            recv(rx) -> freq => { println!("part2: {}", freq.unwrap()); return; } ,
            default(std::time::Duration::from_secs(10)) => println!("Queued: {}, Active: {}", tp.queued_count(), tp.active_count()),
        };
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    //let args: Vec<String> = std::env::args().collect();

    //part1(&lines, 10);
    part1(&lines, 2000000);
    // WRONG: 4184316
    // WRONG: 4207705
    // RIGHT: 5335787
    //
    // yes, I did do it the long way lol, runtime was about 1h 10m on a 2014 era Dell r720 w/
    // 32cores. We'll chalk this one up to "learning about threading" in rust hehe
    part2(&lines, 4_000_000);
    // RIGHT: 13673971349056
}
