use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    children: HashMap<String, Dir>,
    files: Vec<File>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u64,
}

fn part1_and_2(lines: &Vec<String>) {
    /*
    let root = Rc::new(RefCell::new(Dir{
        name: "/".to_string(),
        children: Vec::new(),
        files: Vec::new(),
    }));
    */
    let root = &mut Dir{
        name: "/".to_string(),
        children: HashMap::new(),
        files: Vec::new(),
    };
    let mut dirs = vec!["/"];

    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        /*
        let mut cur = Rc::clone(&root);
        */
        let mut cur: &mut Dir = root;
        for d in dirs.clone() {
            if d == "/" {
                //cur = Rc::clone(&root);
                cur = root;
            } else {
                /*
                    let child = cur.children.iter_mut().filter(|dir| dir.name == d).collect::<Vec<_>>();
                    cur = child[0];
                */
                cur = cur.children.get_mut(d).expect("child dir expected"); //wtf does this work,
                                                                            //but if children is a
                                                                            //vec and I search the
                                                                            //vec and set cur I
                                                                            //have a really really
                                                                            //bad time
            }
        }
        match parts[0] {
            "$" =>
            match parts[1] {
                "cd" =>
                    match parts[2] {
                        "/" => (),
                        ".." => {dirs.pop();},
                        _ => {
                            dirs.push(parts[2]);
                            /*
                            let child = cur.children.iter_mut().filter(|dir| dir.name == parts[2]).collect::<Vec<_>>();
                            cur = child[0];
                            */
                        },
                    }
                "ls" => (),
                _ => (),
            }
            "dir" => {
                let dir: Dir = Dir{
                    name: parts[1].to_string(),
                    children: HashMap::new(),
                    files: Vec::new(),
                };
                cur.children.insert(parts[1].to_string(), dir);
            },
            _ => {
                cur.files.push(File{
                    name: parts[1].to_string(),
                    size: parts[0].parse::<u64>().expect("should have been a num")
                });
            },

        }
    }


    println!("{:#?}", root.clone());
    let dir_sizes = calc_dir(root.clone());
    let root_size = calc_root_dir(root.clone());
    //println!("{:?}", dir_sizes);

    let p1size: u64 = dir_sizes.iter().filter(|d| **d < 100000).sum();
    println!("part1: {}", p1size);

    let unused = 70000000 - root_size;
    let needed = 30000000 - unused;
    let p2size: u64 = *dir_sizes.iter().filter(|d| **d > needed).min().unwrap();
    println!("part2: {}", p2size);
}

fn calc_dir(dir: Dir) -> Vec<u64> {
    let mut dir_sizes: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;
    println!("outer af {} -- {} -- {:?}", dir.name, sum, dir_sizes);
    for child in dir.children.iter() {
        let child_dir_sizes = calc_dir(child.1.clone());
        let child_dir_size = calc_root_dir(child.1.clone());
        sum += child_dir_size;
        dir_sizes.append(&mut child_dir_sizes.clone());
    }
    let dirsum = dir.files.iter().map(|f| f.size).sum::<u64>();
    dir_sizes.push(dirsum + sum);
    dir_sizes
}
fn calc_root_dir(dir: Dir) -> u64 {
    let mut sum: u64 = 0;
    for child in dir.children.iter() {
        let child_dir_size = calc_root_dir(child.1.clone());
        sum += child_dir_size;
    }
    let dirsum = dir.files.iter().map(|f| f.size).sum::<u64>();
    dirsum+sum
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    part1_and_2(&lines);
}
