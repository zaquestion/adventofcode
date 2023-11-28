use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<String>,
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    minutes: usize,
    room: HashMap<usize, String>,
    pressure_relieved: usize,
    valve_states: HashMap<String, bool>,
    path: Vec<Action>,
}

impl State {
    fn new(hm: &HashMap<String, Valve>) -> State {
        let mut states: HashMap<String, bool> = HashMap::new();
        for (k, _) in hm.iter() {
            states.insert(k.to_string(), false);
        }
        State {
            minutes: 0,
            pressure_relieved: 0,
            room: HashMap::from([(0, "AA".to_string()), (1, "AA".to_string())]),
            valve_states: states,
            path: vec![
                Action::Move("M".to_string(), "AA".to_string()),
                Action::Move("E".to_string(), "AA".to_string()),
            ],
        }
    }
}

fn action_to_string(a: &Action) -> String {
    match a {
        Action::Wait(a) => format!("W({})", a).to_string(),
        Action::Move(a, r) => format!("M({},{})", a, r).to_string(),
        Action::Open(a, r) => format!("O({},{})", a, r).to_string(),
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mins: {}, Relieved: {},\n Path: {}\n Valve State: {}",
            self.minutes,
            self.pressure_relieved,
            self.path
                .iter()
                .map(|a| action_to_string(a))
                .collect::<Vec<_>>()
                .join(","),
            self.valve_states
                .iter()
                .map(|(k, v)| if *v {
                    format!("{}:O", k)
                } else {
                    format!("{}:C", k)
                })
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum Action {
    Move(String, String),
    Open(String, String),
    Wait(String),
}

fn part1(lines: &Vec<String>) {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in lines.iter() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let k = parts[1].to_string();

        let flow_rate = parts[4]
            .trim_start_matches("rate=")
            .trim_end_matches(";")
            .parse::<usize>()
            .expect("num");

        let tunnels = &parts[9..]
            .iter()
            .map(|v| v.trim_end_matches(",").to_string())
            .collect::<Vec<_>>();

        valves.insert(
            k,
            Valve {
                flow_rate,
                tunnels: tunnels.to_vec(),
            },
        );
    }
    let mins = 30;

    let start = State::new(&valves);
    let mut biggest_fart: State = start.clone();
    let mut next: Vec<State> = vec![start.clone()];
    while !next.is_empty() {
        let mut nextnext: Vec<State> = Vec::new();
        for state in next.iter_mut() {
            if state.minutes > mins {
                continue;
            }
            if state.pressure_relieved > biggest_fart.pressure_relieved {
                biggest_fart = state.clone();
            }
            if state.valve_states.iter().all(|(_, v)| *v) {
                let mut new = state.clone();
                new.minutes += 1;
                new.pressure_relieved += state
                    .valve_states
                    .iter()
                    .map(|(k, open)| if *open { valves[k].flow_rate } else { 0 })
                    .sum::<usize>();
                new.path.push(Action::Wait("M".to_string()));
                nextnext.push(new);
                continue;
            }

            if !state.valve_states[state.room.get(&0).unwrap()]
                && valves[state.room.get(&0).unwrap()].flow_rate > 0
            {
                let mut new = state.clone();
                new.minutes += 1; // 1 min to open
                new.pressure_relieved += state
                    .valve_states
                    .iter()
                    .map(|(k, open)| if *open { valves[k].flow_rate } else { 0 })
                    .sum::<usize>();
                *new.valve_states
                    .get_mut(state.room.get(&0).unwrap())
                    .unwrap() = true;
                new.path.push(Action::Open(
                    "M".to_string(),
                    state.room.get(&0).unwrap().to_string(),
                ));
                nextnext.push(new);
            }

            for t in valves[state.room.get(&0).unwrap()].tunnels.iter() {
                let mut new = state.clone();
                new.minutes += 1; // one min to move
                *new.room.get_mut(&0).unwrap() = t.to_string();
                new.pressure_relieved += state
                    .valve_states
                    .iter()
                    .map(|(k, open)| if *open { valves[k].flow_rate } else { 0 })
                    .sum::<usize>();
                new.path.push(Action::Move("M".to_string(), t.to_string()));
                nextnext.push(new);
            }
        }
        next = nextnext;
        next.sort_by_key(|s| s.pressure_relieved);
        next.reverse();
        next.resize(min((2 as usize).pow(14), next.len()), State::new(&valves));
    }

    println!("Part 1: {}", biggest_fart.pressure_relieved);
}

fn usize_to_actor(u: usize) -> String {
    match u {
        0 => "M".to_string(),
        1 => "E".to_string(),
        _ => "UNKN".to_string(),
    }
}
fn part2(lines: &Vec<String>) {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in lines.iter() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let k = parts[1].to_string();

        let flow_rate = parts[4]
            .trim_start_matches("rate=")
            .trim_end_matches(";")
            .parse::<usize>()
            .expect("num");

        let tunnels = &parts[9..]
            .iter()
            .map(|v| v.trim_end_matches(",").to_string())
            .collect::<Vec<_>>();

        valves.insert(
            k,
            Valve {
                flow_rate,
                tunnels: tunnels.to_vec(),
            },
        );
    }
    let mins = 26;
    let mut count = 0;

    let start = State::new(&valves);
    let mut biggest_fart: State = start.clone();
    let mut next: Vec<State> = vec![start.clone()];
    for _ in 1..=mins {
        println!("BIGGEST FART: {:?}, {}", biggest_fart, next.len());
        for s in next.iter_mut() {
            s.minutes += 1;
            s.pressure_relieved += s
                .valve_states
                .iter()
                .map(|(k, open)| if *open { valves[k].flow_rate } else { 0 })
                .sum::<usize>();
            if s.pressure_relieved >= biggest_fart.pressure_relieved {
                biggest_fart = s.clone();
            }
        }
        for i in 0..2 {
            let mut nextnext: Vec<State> = Vec::new();
            for state in next.iter_mut() {
                if state.valve_states.iter().all(|(_, v)| *v) {
                    let mut new = state.clone();
                    new.path.push(Action::Wait(usize_to_actor(i)));
                    nextnext.push(new);
                    continue;
                }

                if !state.valve_states[state.room.get(&i).unwrap()] {
                    let mut new = state.clone();
                    *new.valve_states
                        .get_mut(state.room.get(&i).unwrap())
                        .unwrap() = true;
                    new.path.push(Action::Open(
                        usize_to_actor(i),
                        state.room.get(&i).unwrap().to_string(),
                    ));
                    nextnext.push(new);
                }

                for t in valves[state.room.get(&i).unwrap()].tunnels.iter() {
                    let mut new = state.clone();
                    *new.room.get_mut(&i).unwrap() = t.to_string();
                    new.path
                        .push(Action::Move(usize_to_actor(i), t.to_string()));
                    nextnext.push(new);
                }
            }
            next = nextnext;
            next.sort_by_key(|s| s.pressure_relieved);
            next.reverse();
            next.resize(min((2 as usize).pow(16), next.len()), State::new(&valves));
            biggest_fart = next.first().unwrap().clone();
        }
    }
    /*
    for n in next.iter() {
        println!("DEBUG: {:?}", n);
    }
    */

    println!("Part 2: {}", biggest_fart.pressure_relieved);
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    //let input = io::read_to_string(io::stdin()).unwrap();

    //part1(&lines);
    part2(&lines);
    // WRONG: 2662
}
