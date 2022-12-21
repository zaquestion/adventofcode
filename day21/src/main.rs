use crossbeam_channel::bounded;
use std::collections::HashMap;
use std::io;
use std::thread;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Equal(String, String),
    Add(String, String),
    Mult(String, String),
    Divide(String, String),
    Subtract(String, String),
    Yell(isize),
    None,
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    operation: Operation,
    tx: crossbeam_channel::Sender<(String, isize)>,
    rx: crossbeam_channel::Receiver<(String, isize)>,
}

fn part1(monkeys: &Vec<Monkey>) {
    let mut map: HashMap<String, isize> = HashMap::new();
    for m in monkeys.iter() {
        let m = m.clone();
        thread::spawn(move || {
            if let Operation::Yell(i) = m.operation {
                m.tx.send((m.name, i)).unwrap();
            }
        });
    }
    //let tx1 = tx.clone();
    let rx = monkeys[0].rx.clone();

    loop {
        let tx = monkeys[0].tx.clone();
        thread::spawn(move || {
            tx.send(("".to_string(), -1)).unwrap();
        });
        loop {
            let (name, i) = rx.recv().unwrap();
            if name == "" {
                break;
            }
            map.insert(name, i);
        }
        if map.contains_key("root") {
            let answer = map.get("root").unwrap();
            println!("part1: {}", answer);
            break;
        }

        for m in monkeys.iter() {
            let m = m.clone();
            match m.operation {
                Operation::None => (),
                Operation::Yell(..) => (),
                Operation::Add(a, b) => {
                    if map.contains_key(&a) && map.contains_key(&b) && !map.contains_key(&m.name) {
                        let i1 = map.get(&a).unwrap().clone();
                        let i2 = map.get(&b).unwrap().clone();
                        thread::spawn(move || {
                            m.tx.send((m.name, i1 + i2)).unwrap();
                        });
                    }
                }
                Operation::Divide(a, b) => {
                    if map.contains_key(&a) && map.contains_key(&b) && !map.contains_key(&m.name) {
                        let i1 = map.get(&a).unwrap().clone();
                        let i2 = map.get(&b).unwrap().clone();
                        thread::spawn(move || {
                            m.tx.send((m.name, i1 / i2)).unwrap();
                        });
                    }
                }
                Operation::Subtract(a, b) => {
                    if map.contains_key(&a) && map.contains_key(&b) && !map.contains_key(&m.name) {
                        let i1 = map.get(&a).unwrap().clone();
                        let i2 = map.get(&b).unwrap().clone();
                        thread::spawn(move || {
                            m.tx.send((m.name, i1 - i2)).unwrap();
                        });
                    }
                }
                Operation::Mult(a, b) => {
                    if map.contains_key(&a) && map.contains_key(&b) && !map.contains_key(&m.name) {
                        let i1 = map.get(&a).unwrap().clone();
                        let i2 = map.get(&b).unwrap().clone();
                        thread::spawn(move || {
                            m.tx.send((m.name, i1 * i2)).unwrap();
                        });
                    }
                }
                Operation::Equal(_a, _b) => (),
            }
        }
    }
}

fn part2(monkeys: &Vec<Monkey>) {
    let mut map: HashMap<String, isize> = HashMap::new();
    for m in monkeys.iter() {
        if m.name == "humn" {
            continue;
        }
        let m = m.clone();
        thread::spawn(move || {
            if let Operation::Yell(i) = m.operation {
                m.tx.send((m.name, i)).unwrap();
            }
        });
    }
    let rx = monkeys[0].rx.clone();

    loop {
        let tx = monkeys[0].tx.clone();
        thread::spawn(move || {
            tx.send(("".to_string(), -1)).unwrap();
        });
        loop {
            let (name, v) = rx.recv().unwrap();
            if name == "" {
                break;
            }
            map.insert(name, v);
        }
        if map.contains_key("humn") {
            let answer = map.get("humn").unwrap();
            println!("part2: {}", answer);
            break;
        }

        for m in monkeys.iter() {
            let m = m.clone();
            match m.operation {
                Operation::None => (),
                Operation::Yell(..) => (),
                Operation::Add(a, b) => {
                    if map.contains_key(&a) && map.contains_key(&b) && !map.contains_key(&m.name) {
                        let i1 = map.get(&a).unwrap().clone();
                        let i2 = map.get(&b).unwrap().clone();
                        thread::spawn(move || {
                            m.tx.send((m.name, i1 + i2)).unwrap();
                        });
                    }
                }
                Operation::Divide(a, b) => {
                    if map.contains_key(&a) && map.contains_key(&b) && !map.contains_key(&m.name) {
                        let i1 = map.get(&a).unwrap().clone();
                        let i2 = map.get(&b).unwrap().clone();
                        thread::spawn(move || {
                            m.tx.send((m.name, i1 / i2)).unwrap();
                        });
                    }
                }
                Operation::Subtract(a, b) => {
                    if map.contains_key(&a) && map.contains_key(&b) && !map.contains_key(&m.name) {
                        let i1 = map.get(&a).unwrap().clone();
                        let i2 = map.get(&b).unwrap().clone();
                        thread::spawn(move || {
                            m.tx.send((m.name, i1 - i2)).unwrap();
                        });
                    }
                }
                Operation::Mult(a, b) => {
                    if map.contains_key(&a) && map.contains_key(&b) && !map.contains_key(&m.name) {
                        let i1 = map.get(&a).unwrap().clone();
                        let i2 = map.get(&b).unwrap().clone();
                        thread::spawn(move || {
                            m.tx.send((m.name, i1 * i2)).unwrap();
                        });
                    }
                }
                Operation::Equal(a, b) => {
                    if map.contains_key(&a) {
                        let i1 = map.get(&a).unwrap().clone();
                        let toyell = solve(&map, &monkeys.clone(), b, i1);
                        if toyell == isize::MIN {
                            // haven't resolved the monkeys yet
                            thread::sleep(std::time::Duration::from_millis(100));
                            continue;
                        }
                        map.insert("humn".to_string(), toyell);
                    } else if map.contains_key(&b) {
                        let i2 = map.get(&b).unwrap().clone();
                        let toyell = solve(&map, &monkeys.clone(), a, i2);
                        if toyell == isize::MIN {
                            // haven't resolved the monkeys yet
                            thread::sleep(std::time::Duration::from_millis(100));
                            continue;
                        }
                        map.insert("humn".to_string(), toyell);
                    }
                }
            }
        }
    }
}

fn solve(map: &HashMap<String, isize>, monkeys: &Vec<Monkey>, k: String, toget: isize) -> isize {
    if k == "humn" {
        return toget;
    }
    let m = monkeys.iter().find(|&mk| mk.name == k).unwrap();
    match &m.operation {
        Operation::Add(a, b) => {
            if map.contains_key(a) {
                // a + b = toget
                // 2 + 4 = 6
                // toget - a = b
                // 6 - 2 = 4
                let i1 = map.get(a).unwrap().clone();
                return solve(&map, &monkeys, b.to_string(), toget - i1);
            } else if map.contains_key(b) {
                // a + b = toget
                // 2 + 4 = 6
                // toget - b = a
                // 6 - 4 = 2
                let i2 = map.get(b).unwrap().clone();
                return solve(&map, &monkeys, a.to_string(), toget - i2);
            }
        }
        Operation::Divide(a, b) => {
            if map.contains_key(a) {
                // a / b = toget
                // 8 / 2 = 4
                // a / toget = b
                // 8 / 4 = 2
                let i1 = map.get(a).unwrap().clone();
                return solve(&map, &monkeys, a.to_string(), i1 / toget);
            } else if map.contains_key(b) {
                // a / b = toget
                // 8 / 2 = 4
                // toget * b = a
                // 4  * 2 = 8
                let i2 = map.get(b).unwrap().clone();
                return solve(&map, &monkeys, a.to_string(), toget * i2);
            }
        }
        Operation::Subtract(a, b) => {
            if map.contains_key(a) {
                // a - b = toget
                // 5 - 3 = 2
                // a - toget = b
                // 5 - 2 = 3
                let i1 = map.get(a).unwrap().clone();
                return solve(&map, &monkeys, b.to_string(), i1 - toget);
            } else if map.contains_key(b) {
                // a - b = toget
                // 5 - 3 = 2
                // toget + b = a
                // 2 + 3 = 5
                let i2 = map.get(b).unwrap().clone();
                return solve(&map, &monkeys, a.to_string(), toget + i2);
            }
        }
        Operation::Mult(a, b) => {
            if map.contains_key(a) {
                // a * b = toget
                // 2 * 4 = 8
                // toget / a = b
                // 8 / 2 = 4
                let i1 = map.get(a).unwrap().clone();
                return solve(&map, &monkeys, b.to_string(), toget / i1);
            } else if map.contains_key(b) {
                // a * b = toget
                // 2 * 4 = 8
                // toget / b = a
                // 8 / 4 = 2
                let i2 = map.get(b).unwrap().clone();
                return solve(&map, &monkeys, a.to_string(), toget / i2);
            }
        }
        _ => (),
    }
    return isize::MIN;
}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    let (tx, rx) = bounded::<(String, isize)>(10);
    for line in lines {
        let parts = line.split(':').collect::<Vec<_>>();
        let op = parts[1].trim().split_whitespace().collect::<Vec<_>>();
        if op.len() > 1 {
            let operation = match op[1] {
                "+" => Operation::Add(op[0].to_string(), op[2].to_string()),
                "-" => Operation::Subtract(op[0].to_string(), op[2].to_string()),
                "/" => Operation::Divide(op[0].to_string(), op[2].to_string()),
                "*" => Operation::Mult(op[0].to_string(), op[2].to_string()),
                _ => Operation::None,
            };
            monkeys.push(Monkey {
                name: parts[0].to_string(),
                operation,
                tx: tx.clone(),
                rx: rx.clone(),
            });
        } else {
            monkeys.push(Monkey {
                name: parts[0].to_string(),
                operation: Operation::Yell(op[0].parse::<isize>().expect("must num")),
                tx: tx.clone(),
                rx: rx.clone(),
            });
        }
    }
    part1(&monkeys);

    let monkeys2 = &mut monkeys.clone();
    for i in 0..monkeys2.len() {
        if monkeys2[i].name == "root" {
            if let Operation::Add(a, b) = &monkeys2[i].operation {
                monkeys2[i].operation = Operation::Equal(a.to_string(), b.to_string());
                break;
            }
        }
    }

    part2(monkeys2);
}
