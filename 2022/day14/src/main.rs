use std::io;

#[derive(Clone, PartialEq)]
enum Cell {
    Rock,
    Air,
    Sand,
    SandSpawner,
}
fn setup_cave(lines: &Vec<String>, fill: bool) -> (Vec<Vec<Cell>>, usize, usize, usize, usize) {
    let mut rocks: Vec<Vec<(usize, usize)>> = Vec::new();
    // get rock connections
    for line in lines {
        let rock = line
            .split("->")
            .into_iter()
            .map(|xy| xy.trim())
            .map(|xy| xy.split(',').into_iter())
            .map(|mut xy_i| {
                (
                    xy_i.next().unwrap().parse::<usize>().expect("must num"),
                    xy_i.next().unwrap().parse::<usize>().expect("must num"),
                )
            })
            .collect::<Vec<_>>();

        rocks.push(rock);
    }
    // find dimensions
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;
    rocks.iter().for_each(|rock| {
        let mix = rock.iter().map(|slab| slab.0).reduce(usize::min).unwrap();
        if mix < min_x {
            min_x = mix;
        }
        let miy = rock.iter().map(|slab| slab.1).reduce(usize::min).unwrap();
        if miy < min_y {
            min_y = miy;
        }
        let max = rock.iter().map(|slab| slab.0).reduce(usize::max).unwrap();
        if max > max_x {
            max_x = max;
        }
        let may = rock.iter().map(|slab| slab.1).reduce(usize::max).unwrap();
        if may > max_y {
            max_y = may;
        }
    });

    let (mut w, mut h) = (max_x - min_x + 1, max_y + 1);
    if fill {
        h += 2;
        w = h * 2 + 1;
    }
    let offset;
    if fill {
        offset = w / 2 - (500 - min_x);
    } else {
        offset = 0;
    }

    // init cave slabs
    let mut cave: Vec<Vec<Cell>> = vec![vec![Cell::Air; w]; h];
    cave[0][500 - min_x + offset] = Cell::SandSpawner;
    for rock in rocks.iter() {
        for slabs in rock.windows(2).into_iter() {
            if slabs[0].0 == slabs[1].0 {
                if slabs[0].1 < slabs[1].1 {
                    for y in slabs[0].1..slabs[1].1 {
                        cave[y][slabs[0].0 - min_x + offset] = Cell::Rock;
                    }
                } else {
                    for y in slabs[1].1..slabs[0].1 {
                        cave[y][slabs[0].0 - min_x + offset] = Cell::Rock;
                    }
                }
                continue;
            }
            if slabs[0].1 == slabs[1].1 {
                if slabs[0].0 < slabs[1].0 {
                    for x in slabs[0].0..slabs[1].0 + 1 {
                        cave[slabs[0].1][x - min_x + offset] = Cell::Rock;
                    }
                } else {
                    for x in slabs[1].0..slabs[0].0 + 1 {
                        cave[slabs[0].1][x - min_x + offset] = Cell::Rock;
                    }
                }
                continue;
            }
        }
    }

    if fill {
        for i in 0..w {
            cave[h - 1][i] = Cell::Rock;
        }
    }

    (cave, w, h, min_x, offset)
}
struct State {
    min_x: usize,
    offset: usize,
    sand_falling: bool,
    x: isize,
    y: isize,
    grains: usize,
}

fn update(cave: &mut Vec<Vec<Cell>>, state: &mut State, fill: bool) -> bool {
    let (x, y) = (state.x as usize, state.y as usize);
    if !state.sand_falling {
        state.x = 500 - state.min_x as isize + state.offset as isize;
        state.y = 0;

        state.grains += 1;
        state.sand_falling = true;
        return true;
    }

    if !fill
        && (state.x - 1 < 0
            || state.x + 1 > cave[0].len() as isize - 1
            || state.y + 1 > cave.len() as isize - 1)
    {
        cave[y][x] = Cell::Air;
        state.grains -= 1;
        return false;
    }

    if cave[y + 1][x] == Cell::Air && cave[y + 1][x] != Cell::Rock && cave[y + 1][x] != Cell::Sand {
        state.y += 1;
    } else if cave[y + 1][x - 1] == Cell::Air
        && cave[y + 1][x - 1] != Cell::Rock
        && cave[y + 1][x - 1] != Cell::Sand
    {
        state.x -= 1;
        state.y += 1;
    } else if cave[y + 1][x + 1] == Cell::Air
        && cave[y + 1][x + 1] != Cell::Rock
        && cave[y + 1][x + 1] != Cell::Sand
    {
        state.x += 1;
        state.y += 1;
    } else {
        //sand at rest
        state.sand_falling = false;
        if fill && state.x as usize == 500 - state.min_x + state.offset && state.y == 0 {
            return false;
        }
    }

    cave[y][x] = Cell::Air;
    let (x, y) = (state.x as usize, state.y as usize);
    cave[y][x] = Cell::Sand;
    cave[0][500 - state.min_x + state.offset] = Cell::SandSpawner;
    true
}

fn render(cave: &Vec<Vec<Cell>>, state: &State) {
    println!("\x1Bc");
    for (i, row) in cave.iter().enumerate() {
        print!("{:3}: ", i);
        for col in row.iter() {
            match col {
                Cell::Air => print!("."),
                Cell::Rock => print!("#"),
                Cell::Sand => print!("*"),
                Cell::SandSpawner => print!("+"),
            };
        }
        println!();
    }
    println!("Cur: {},{}, Grains: {}", state.x, state.y, state.grains);
}

fn part1(lines: &Vec<String>) {
    let (c, _w, _h, min_x, offset) = setup_cave(lines, false);
    let mut cave = c;

    let mut state = State {
        min_x,
        offset,
        sand_falling: false,
        x: 500 - min_x as isize + offset as isize,
        y: 0,
        grains: 0,
    };
    // game loop
    loop {
        let running = update(&mut cave, &mut state, false);
        if !running {
            break;
        }
        //std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    render(&cave, &state);
}

fn part2(lines: &Vec<String>) {
    let (c, _w, _h, min_x, offset) = setup_cave(lines, true);
    let mut cave = c;

    let mut state = State {
        min_x,
        offset,
        sand_falling: false,
        x: 500 - min_x as isize + offset as isize,
        y: 0,
        grains: 0,
    };
    // game loop
    loop {
        let running = update(&mut cave, &mut state, true);
        if !running {
            break;
        }
    }
    render(&cave, &state);
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    //let input = io::read_to_string(io::stdin()).unwrap();

    part1(&lines);
    part2(&lines);
}
