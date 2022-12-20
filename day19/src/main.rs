use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::io;

#[derive(Debug)]
struct Blueprint {
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    minutes: usize,
    bp_num: usize,
    resources: HashMap<Resource, usize>,
    robots: HashMap<Resource, usize>,
    path_per_minute: Vec<Resource>, //robot made
}

impl State {
    fn new(bp_num: usize) -> State {
        State {
            minutes: 0,
            bp_num,
            resources: HashMap::from([
                (Resource::Ore, 0),
                (Resource::Clay, 0),
                (Resource::Obsidian, 0),
                (Resource::Geode, 0),
            ]),
            robots: HashMap::from([
                (Resource::Ore, 1),
                (Resource::Clay, 0),
                (Resource::Obsidian, 0),
                (Resource::Geode, 0),
            ]),
            path_per_minute: Vec::new(),
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Mins: {}, Quality: {},\n Robots: {:?},\n Resources: {},\n Path: {}",
            self.minutes,
            calc_quality(self),
            self.robots
                .iter()
                .map(|(k, v)| format!("{}: {}", resource_to_str(k), v))
                .collect::<Vec<_>>()
                .join(", "),
            self.resources
                .iter()
                .map(|(k, v)| format!("{}: {}", resource_to_str(k), v))
                .collect::<Vec<_>>()
                .join(", "),
            self.path_per_minute
                .iter()
                .map(|r| resource_to_str(r))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

fn resource_to_str(r: &Resource) -> &str {
    match r {
        Resource::Wait => "W",

        Resource::Ore => "O",
        Resource::Clay => "C",
        Resource::Obsidian => "Ob",
        Resource::Geode => "G",
    }
}

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
    Wait,
}

const RESOURCES: [Resource; 4] = [
    Resource::Ore,
    Resource::Clay,
    Resource::Obsidian,
    Resource::Geode,
];

fn part1(lines: &Vec<String>) {
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in lines.iter() {
        //line.split("Each").collect::Vec<_>();
        let parts = line
            .split_whitespace()
            .filter_map(|w| w.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        blueprints.push(Blueprint {
            ore_robot_ore_cost: parts[0],
            clay_robot_ore_cost: parts[1],
            obsidian_robot_ore_cost: parts[2],
            obsidian_robot_clay_cost: parts[3],
            geode_robot_ore_cost: parts[4],
            geode_robot_obsidian_cost: parts[5],
        });
    }
    //println!("{:?}", blueprints);

    let mins = 24;
    let mut count = 0;

    let mut bp_qualities = 0;
    for (i, bp) in blueprints.iter().enumerate() {
        let start = State::new(i + 1);
        let mut top_quality: State = start.clone();
        let mut next: Vec<State> = vec![start.clone()];
        while !next.is_empty() {
            let mut nextnext: Vec<State> = Vec::new();
            for state in next.iter_mut() {
                if state.minutes > mins {
                    continue;
                }
                if calc_quality(state) > calc_quality(&top_quality) {
                    top_quality = state.clone();
                }

                if state.resources[&Resource::Ore] >= bp.ore_robot_ore_cost {
                    let mut new = state.clone();
                    *new.resources.get_mut(&Resource::Ore).unwrap() -= bp.ore_robot_ore_cost;
                    new.path_per_minute.push(Resource::Ore);
                    new.minutes += 1;
                    add_resources(&mut new);
                    *new.robots.get_mut(&Resource::Ore).unwrap() += 1;
                    nextnext.push(new);
                }

                if state.resources[&Resource::Ore] >= bp.clay_robot_ore_cost {
                    let mut new = state.clone();
                    *new.resources.get_mut(&Resource::Ore).unwrap() -= bp.clay_robot_ore_cost;
                    new.path_per_minute.push(Resource::Clay);
                    new.minutes += 1;
                    add_resources(&mut new);
                    *new.robots.get_mut(&Resource::Clay).unwrap() += 1;
                    nextnext.push(new);
                }

                if state.resources[&Resource::Ore] >= bp.obsidian_robot_ore_cost
                    && state.resources[&Resource::Clay] >= bp.obsidian_robot_clay_cost
                {
                    let mut new = state.clone();
                    *new.resources.get_mut(&Resource::Ore).unwrap() -= bp.obsidian_robot_ore_cost;
                    *new.resources.get_mut(&Resource::Clay).unwrap() -= bp.obsidian_robot_clay_cost;
                    new.path_per_minute.push(Resource::Obsidian);
                    new.minutes += 1;
                    add_resources(&mut new);
                    *new.robots.get_mut(&Resource::Obsidian).unwrap() += 1;
                    nextnext.push(new);
                }

                if state.resources[&Resource::Ore] >= bp.geode_robot_ore_cost
                    && state.resources[&Resource::Obsidian] >= bp.geode_robot_obsidian_cost
                {
                    let mut new = state.clone();
                    *new.resources.get_mut(&Resource::Ore).unwrap() -= bp.geode_robot_ore_cost;
                    *new.resources.get_mut(&Resource::Obsidian).unwrap() -=
                        bp.geode_robot_obsidian_cost;
                    new.path_per_minute.push(Resource::Geode);
                    new.minutes += 1;
                    add_resources(&mut new);
                    *new.robots.get_mut(&Resource::Geode).unwrap() += 1;
                    nextnext.push(new);
                }

                state.path_per_minute.push(Resource::Wait);
                let mut new = state.clone();
                add_resources(&mut new);
                new.minutes += 1;
                nextnext.push(new);
            }
            next = nextnext;
            next.sort_by(|a, b| {
                if a.resources[&Resource::Geode] > b.resources[&Resource::Geode]
                    || a.robots[&Resource::Geode] > b.robots[&Resource::Geode]
                {
                    std::cmp::Ordering::Greater
                } else if (a.resources[&Resource::Geode] == b.resources[&Resource::Geode]
                    && a.robots[&Resource::Geode] == b.robots[&Resource::Geode])
                    && (a.resources[&Resource::Obsidian] > b.resources[&Resource::Obsidian]
                        || a.robots[&Resource::Obsidian] > b.robots[&Resource::Obsidian])
                {
                    std::cmp::Ordering::Greater
                } else if (a.resources[&Resource::Geode] == b.resources[&Resource::Geode]
                    && a.robots[&Resource::Geode] == b.robots[&Resource::Geode])
                    && (a.resources[&Resource::Obsidian] == b.resources[&Resource::Obsidian]
                        && a.robots[&Resource::Obsidian] == b.robots[&Resource::Obsidian])
                    && (a.resources[&Resource::Clay] > b.resources[&Resource::Clay]
                        || a.robots[&Resource::Clay] > b.robots[&Resource::Clay])
                {
                    std::cmp::Ordering::Greater
                } else if (a.resources[&Resource::Geode] == b.resources[&Resource::Geode]
                    && a.robots[&Resource::Geode] == b.robots[&Resource::Geode])
                    && (a.resources[&Resource::Obsidian] == b.resources[&Resource::Obsidian]
                        && a.robots[&Resource::Obsidian] == b.robots[&Resource::Obsidian])
                    && (a.resources[&Resource::Clay] == b.resources[&Resource::Clay]
                        && a.robots[&Resource::Clay] == b.robots[&Resource::Clay])
                    && (a.resources[&Resource::Ore] > b.resources[&Resource::Ore]
                        || a.robots[&Resource::Ore] > b.robots[&Resource::Ore])
                {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            next.reverse();
            next.resize(min((2 as usize).pow(8), next.len()), State::new(i + 1));
            /*
            next.retain(|n| {
                n.resources[&Resource::Geode] >= top_quality.resources[&Resource::Geode]
            });
            for n in next.iter() {
                //if n.resources[&Resource::Geode] > 0 {
                if n.path_per_minute.contains(&Resource::Clay) {
                    println!("DEBUG: bp: {}, {:?}, nnlen: {}", n.bp_num, n, next.len());
                }
            }
            println!("BP: {} -> TOP QUALITY {:?}", i + 1, top_quality);
            */
            count += 1;
        }

        bp_qualities += calc_quality(&top_quality);
    }
    println!("Part 1: {}", bp_qualities);
}

fn calc_quality(state: &State) -> usize {
    state.resources[&Resource::Geode] * (state.bp_num)
}

fn add_resources(state: &mut State) {
    for r in &RESOURCES {
        *state.resources.get_mut(&r).unwrap() += state.robots[&r];
    }
}

fn part2(lines: &Vec<String>) {
    let mut blueprints: Vec<Blueprint> = Vec::new();
    for line in lines.iter() {
        //line.split("Each").collect::Vec<_>();
        let parts = line
            .split_whitespace()
            .filter_map(|w| w.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        blueprints.push(Blueprint {
            ore_robot_ore_cost: parts[0],
            clay_robot_ore_cost: parts[1],
            obsidian_robot_ore_cost: parts[2],
            obsidian_robot_clay_cost: parts[3],
            geode_robot_ore_cost: parts[4],
            geode_robot_obsidian_cost: parts[5],
        });
    }
    //println!("{:?}", blueprints);

    let mins = 32;
    let mut count = 0;

    let mut geodes: Vec<usize> = Vec::new();
    for (i, bp) in blueprints.iter().take(3).enumerate() {
        let start = State::new(i + 1);
        let mut top_quality: State = start.clone();
        let mut next: Vec<State> = vec![start.clone()];
        while !next.is_empty() {
            let mut nextnext: Vec<State> = Vec::new();
            for state in next.iter_mut() {
                if state.minutes > mins {
                    continue;
                }
                if calc_quality(state) > calc_quality(&top_quality) {
                    top_quality = state.clone();
                }

                if state.resources[&Resource::Ore] >= bp.ore_robot_ore_cost {
                    let mut new = state.clone();
                    *new.resources.get_mut(&Resource::Ore).unwrap() -= bp.ore_robot_ore_cost;
                    new.path_per_minute.push(Resource::Ore);
                    new.minutes += 1;
                    add_resources(&mut new);
                    *new.robots.get_mut(&Resource::Ore).unwrap() += 1;
                    nextnext.push(new);
                }

                if state.resources[&Resource::Ore] >= bp.clay_robot_ore_cost {
                    let mut new = state.clone();
                    *new.resources.get_mut(&Resource::Ore).unwrap() -= bp.clay_robot_ore_cost;
                    new.path_per_minute.push(Resource::Clay);
                    new.minutes += 1;
                    add_resources(&mut new);
                    *new.robots.get_mut(&Resource::Clay).unwrap() += 1;
                    nextnext.push(new);
                }

                if state.resources[&Resource::Ore] >= bp.obsidian_robot_ore_cost
                    && state.resources[&Resource::Clay] >= bp.obsidian_robot_clay_cost
                {
                    let mut new = state.clone();
                    *new.resources.get_mut(&Resource::Ore).unwrap() -= bp.obsidian_robot_ore_cost;
                    *new.resources.get_mut(&Resource::Clay).unwrap() -= bp.obsidian_robot_clay_cost;
                    new.path_per_minute.push(Resource::Obsidian);
                    new.minutes += 1;
                    add_resources(&mut new);
                    *new.robots.get_mut(&Resource::Obsidian).unwrap() += 1;
                    nextnext.push(new);
                }

                if state.resources[&Resource::Ore] >= bp.geode_robot_ore_cost
                    && state.resources[&Resource::Obsidian] >= bp.geode_robot_obsidian_cost
                {
                    let mut new = state.clone();
                    *new.resources.get_mut(&Resource::Ore).unwrap() -= bp.geode_robot_ore_cost;
                    *new.resources.get_mut(&Resource::Obsidian).unwrap() -=
                        bp.geode_robot_obsidian_cost;
                    new.path_per_minute.push(Resource::Geode);
                    new.minutes += 1;
                    add_resources(&mut new);
                    *new.robots.get_mut(&Resource::Geode).unwrap() += 1;
                    nextnext.push(new);
                }

                state.path_per_minute.push(Resource::Wait);
                let mut new = state.clone();
                add_resources(&mut new);
                new.minutes += 1;
                nextnext.push(new);
            }
            next = nextnext;
            next.sort_by(|a, b| {
                if a.resources[&Resource::Geode] > b.resources[&Resource::Geode]
                    || a.robots[&Resource::Geode] > b.robots[&Resource::Geode]
                {
                    std::cmp::Ordering::Greater
                } else if (a.resources[&Resource::Geode] == b.resources[&Resource::Geode]
                    && a.robots[&Resource::Geode] == b.robots[&Resource::Geode])
                    && (a.resources[&Resource::Obsidian] > b.resources[&Resource::Obsidian]
                        || a.robots[&Resource::Obsidian] > b.robots[&Resource::Obsidian])
                {
                    std::cmp::Ordering::Greater
                } else if (a.resources[&Resource::Geode] == b.resources[&Resource::Geode]
                    && a.robots[&Resource::Geode] == b.robots[&Resource::Geode])
                    && (a.resources[&Resource::Obsidian] == b.resources[&Resource::Obsidian]
                        && a.robots[&Resource::Obsidian] == b.robots[&Resource::Obsidian])
                    && (a.resources[&Resource::Clay] > b.resources[&Resource::Clay]
                        || a.robots[&Resource::Clay] > b.robots[&Resource::Clay])
                {
                    std::cmp::Ordering::Greater
                } else if (a.resources[&Resource::Geode] == b.resources[&Resource::Geode]
                    && a.robots[&Resource::Geode] == b.robots[&Resource::Geode])
                    && (a.resources[&Resource::Obsidian] == b.resources[&Resource::Obsidian]
                        && a.robots[&Resource::Obsidian] == b.robots[&Resource::Obsidian])
                    && (a.resources[&Resource::Clay] == b.resources[&Resource::Clay]
                        && a.robots[&Resource::Clay] == b.robots[&Resource::Clay])
                    && (a.resources[&Resource::Ore] > b.resources[&Resource::Ore]
                        || a.robots[&Resource::Ore] > b.robots[&Resource::Ore])
                {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });
            next.reverse();
            next.resize(min((2 as usize).pow(14), next.len()), State::new(i + 1));
            /*
            next.retain(|n| {
                n.resources[&Resource::Geode] >= top_quality.resources[&Resource::Geode]
            });
            for n in next.iter() {
                //if n.resources[&Resource::Geode] > 0 {
                if n.path_per_minute.contains(&Resource::Clay) {
                    println!("DEBUG: bp: {}, {:?}, nnlen: {}", n.bp_num, n, next.len());
                }
            }
            println!("BP: {} -> TOP QUALITY {:?}", i + 1, top_quality);
            */
            count += 1;
        }
        println!(
            "BP: {}, Geodes: {}",
            i + 1,
            top_quality.resources[&Resource::Geode]
        );
        geodes.push(top_quality.resources[&Resource::Geode]);
    }
    println!("Part 2: {}", geodes.iter().product::<usize>());
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();

    part1(&lines);
    part2(&lines);
}
