use std::io;

#[derive(Debug,PartialEq,Clone)]
struct Monkey {
    starting_items: Vec<usize>,
    operation: Operation,
    divisible: usize,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug,PartialEq,Clone)]
enum Operation {
    AddX(usize),
    MultX(usize),
    MultOld,
    None
}

fn parse_input_lines(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    // Split the input into lines
    let chunks: Vec<&str> = input.split("\n\n").collect();

    // Parse each line and build the corresponding Monkey object
    for chunk in chunks {
        let lines = chunk.lines().collect::<Vec<&str>>();
        let mut line_iter = lines.iter();
        let _line = line_iter.next();

        // starting
        let line = line_iter.next().unwrap();
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let starting: Vec<usize> = parts.iter().skip(2).map(|p| p.trim_end_matches(",").parse::<usize>().expect("must number")).collect();


        let line = line_iter.next().unwrap();
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let op = match parts[4] {
            "*" => match parts[5] {
                "old" => Operation::MultOld,
                _ => Operation::MultX(parts[5].parse::<usize>().expect("must num")),
            }
            "+" => Operation::AddX(parts[5].parse::<usize>().expect("must num")),
            _ => Operation::None,
        };

        //test
        let line = line_iter.next().unwrap();
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let divisible = parts[3].parse::<usize>().expect("must num");

        //if1
        let line = line_iter.next().unwrap();
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let if_true = parts[5].parse::<usize>().expect("must num");

        //if2
        let line = line_iter.next().unwrap();
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let if_false = parts[5].parse::<usize>().expect("must num");

        monkeys.push(Monkey{
            starting_items: starting,
            operation: op,
            divisible,
            if_true,
            if_false,
        })
    }

    monkeys
}

fn simulate_rounds(
    monkeys: Vec<Monkey>,
    rounds: usize,
    part1: bool,
) -> u128 {
    let mut monkeyinspects: [u128; 8] = [0; 8];
    let mut monkeysclone = monkeys.to_vec();

    for _ in 1..=rounds {
        let md: usize = monkeys.iter().map(|monkey| monkey.divisible).product();
        for idx in 0..monkeysclone.len(){
            let m = monkeysclone[idx].clone();
            for item in &m.starting_items {
                let mut new_worry_level = match m.operation {
                    Operation::AddX(x) => item + x,
                    Operation::MultX(x) => item * x % md,
                    Operation::MultOld => item * item % md,
                    Operation::None => *item,
                };

                if part1 {
                    new_worry_level /= 3;
                } else {
                }

                if new_worry_level % m.divisible == 0 {
                    let mut newvec = monkeysclone[m.if_true].starting_items.clone();
                    newvec.push(new_worry_level);
                    let index = monkeysclone[idx].starting_items.iter().position(|&i| i == *item).unwrap();
                    monkeysclone[idx].starting_items.remove(index);
                    monkeysclone[m.if_true].starting_items = newvec;
                } else {
                    let mut newvec = monkeysclone[m.if_false].starting_items.clone();
                    newvec.push(new_worry_level);
                    let index = monkeysclone[idx].starting_items.iter().position(|&i| i == *item).unwrap();
                    monkeysclone[idx].starting_items.remove(index);
                    monkeysclone[m.if_false].starting_items = newvec;
                }
                monkeyinspects[idx] += 1
            }
        }
        /*
        println!("Round {}", r);
        for m in monkeysclone.clone().iter() {
            println!("{:?}", m);
        }
        println!();
        */
    }

    monkeyinspects.sort();

    println!("{:?}", monkeyinspects);
    let top_two = monkeyinspects.iter().rev().take(2).collect::<Vec<_>>();
    top_two[0] * top_two[1]
}

fn part1(input: &str) {
    let monkeys = parse_input_lines(&input);
    println!("{:?}", simulate_rounds(monkeys, 20, true));
}

fn part2(input: &str) {
    let monkeys = parse_input_lines(&input);
    println!("{:?}", simulate_rounds(monkeys, 10000, false));
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();

    part1(&input);
    part2(&input);
}
