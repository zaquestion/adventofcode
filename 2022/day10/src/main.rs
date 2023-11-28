use std::io;


fn part1(lines: &Vec<String>) {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "addx" => {
                let num = parts[1].parse::<i64>().unwrap();
                instructions.push(Instruction::AddX(num));
              }
            "noop" => {
                instructions.push(Instruction::Noop);
                }
            _ => ()
            }
        }
    let mut x = 1;
    let mut cycle = 1;
    let mut sum = 0;

    for instruction in instructions.iter() {
        cycle += 1;
        if   cycle == 20 || cycle % 40 == 20 {
            let signal_strength = cycle as i64 * x;
            sum += signal_strength;
            println!("Signal strength at cycle {} -- x->{}: {}", cycle, x, signal_strength);
        }

        match instruction {
            Instruction::Noop => { }
            Instruction::AddX(v) => {
                x += v;
                cycle += 1;
                if cycle == 20 || cycle % 40 == 20 {
                    let signal_strength = cycle as i64 * x;
                    sum += signal_strength;
                    println!("Signal strength at cycle {} -- x->{}: {}", cycle, x, signal_strength);
                }
            }
        }


    }
    println!("{}", sum);
}

enum Instruction {
    Noop,
    AddX(i64),
}

fn part2(lines: &Vec<String>) {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "addx" => {
                let num = parts[1].parse::<i64>().unwrap();
                instructions.push(Instruction::AddX(num));
              }
            "noop" => {
                instructions.push(Instruction::Noop);
                }
            _ => ()
            }
        }
    let mut x = 1;
    let mut cycle = 0;

    let cyclex = cycle % 40;
    if (cyclex-1..=cyclex+1).contains(&x) {
        print!("#");
    } else {
        print!(".");
    }

    for instruction in instructions.iter() {
        cycle += 1;
        let cyclex = cycle % 40;
        if cycle % 40 == 0 {
            print!("\n");
        }
        if (cyclex-1..=cyclex+1).contains(&x) {
            print!("#");
        } else {
            print!(".");
        }

        match instruction {
            Instruction::Noop => { }
            Instruction::AddX(v) => {
                x += v;
                cycle += 1;
                let cyclex = cycle % 40;
                if cycle % 40 == 0 {
                    print!("\n");
                }
                if (cyclex-1..=cyclex+1).contains(&x) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    part1(&lines);
    part2(&lines);
}
