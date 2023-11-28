use std::io;

fn tetris(input: &str, rock_count: usize) -> usize {
    // Initialize the chamber grid with all cells empty
    let mut chamber = vec![vec!['.'; 7]; 0];

    // Initialize the list of rocks
    let rocks = vec![
        vec![
            "####", //
        ],
        vec![
            ".#.", //
            "###", //
            ".#.", //
        ],
        vec![
            "..#", //
            "..#", //
            "###", //
        ],
        vec![
            "#", //
            "#", //
            "#", //
            "#", //
        ],
        vec![
            "##", //
            "##", //
        ],
    ];

    let jet_pattern = input.trim();

    let mut jet_idx = 0;
    let mut rock_idx = 0;
    let mut last_height = 0;
    let mut last_rockn = 0;

    for rockn in 0..rock_count {
        let rock = &rocks[rock_idx];
        let mut x = 2;
        let mut y = chamber.len() + 3 - 1 + rock.len();

        loop {
            x = match jet_pattern.chars().nth(jet_idx).unwrap() {
                '<' => {
                    if in_bounds(&chamber, rock.to_vec(), x, y, -1, 0) {
                        x - 1
                    } else {
                        x
                    }
                }
                '>' => {
                    if in_bounds(&chamber, rock.to_vec(), x, y, 1, 0) {
                        x + 1
                    } else {
                        x
                    }
                }
                _ => 0,
            };

            jet_idx = (jet_idx + 1) % jet_pattern.len();

            if in_bounds(&chamber, rock.to_vec(), x, y, 0, -1) {
                y -= 1
            } else {
                manifest_rock(&mut chamber, rock.to_vec(), x, y);
                //print_chamber(&chamber);
                break;
            }

            if jet_idx == 0 {
                println!(
                    "rock: {}, height: {}, rdelta: {}, hdelta: {}, pct: {:.03}, gr: {}, gh: {}",
                    rockn,
                    chamber.len(),
                    rockn - last_rockn,
                    chamber.len() - last_height,
                    rockn as f64 / rock_count as f64 * 100.0,
                    1688 + (((rock_count / 1690) - 1) * 1690) + 291,
                    2645 + (((rock_count / 1690) - 1) * 2647) + 442,
                );
                //rock: 1688, height: 2645, rdelta: 1688, hdelta: 2645, pct: 0.000
                //rock: 1688, height: 2645, rdelta: 1688, hdelta: 2645, pct: 0.000, gr: 999999999438, gh: 1566272188470

                //$ echo $((1000000000000-999999999438-1))
                //561
                //

                //if rockn == 1688 + 561 {
                //rock: 2249, height: 3528, rdelta: 561, hdelta: 882
                //
                //
                //WRONG: 1566272189353
                //RIGHT!: 1566272189352

                last_height = chamber.len();
                last_rockn = rockn
            }
        }
        if rockn == 1688 + 561 || // part2 remaining
                rockn == 1688 + 291 || // 100000 test base & below
                rockn == 100000 - 1
        {
            println!(
                "rock: {}, height: {}, rdelta: {}, hdelta: {}",
                rockn,
                chamber.len(),
                rockn - last_rockn,
                chamber.len() - last_height,
            );
        }
        rock_idx = (rock_idx + 1) % rocks.len();
    }
    chamber.len()
}

fn in_bounds(
    chamber: &Vec<Vec<char>>,
    r: Vec<&str>,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> bool {
    if x as isize + dx < 0 {
        return false;
    }
    if x as isize + r[0].len() as isize + dx > 7 {
        return false;
    }
    if y as isize - r.len() as isize + 1 + dy < 0 {
        return false;
    }
    for ry in 0..r.len() {
        let ty = (y as isize - ry as isize + dy) as usize;
        for rx in 0..r[ry].len() {
            if r[ry].chars().nth(rx) == Some('.') {
                continue;
            }
            let tx = (x as isize + rx as isize + dx) as usize;
            if chamber.len() > ty && chamber[ty][tx] == '#' {
                return false;
            }
        }
    }
    return true;
}

fn manifest_rock(chamber: &mut Vec<Vec<char>>, r: Vec<&str>, x: usize, y: usize) {
    while y >= chamber.len() {
        chamber.push(vec!['.'; 7]);
    }
    for ry in 0..r.len() {
        let ty = y - ry;
        for rx in 0..r[ry].len() {
            if r[ry].chars().nth(rx) == Some('.') {
                continue;
            }
            let tx = x + rx;
            chamber[ty][tx] = r[ry].chars().nth(rx).unwrap();
        }
    }
}

fn main() {
    //let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    let input = io::read_to_string(io::stdin()).unwrap();

    println!("part 1: {}", tetris(&input, 2022)); // 2022 pt 1

    println!("part 2 test: {}", tetris(&input, 100000));

    println!(
        "part 2: {}",
        2645 + (((1000000000000 as i64 / 1690) - 1) * 2647) + 882
    ); // 1000000000000 pt 2
       //
       //
}

fn print_chamber(chamber: &Vec<Vec<char>>) {
    println!("\x1Bc");
    for (i, row) in chamber.iter().enumerate().rev() {
        print!("{:2}: |", i);
        for c in row.iter() {
            print!("{}", c);
        }
        println!("|");
    }
    println!("    +-------+");
    std::thread::sleep(std::time::Duration::from_millis(1000));
}
