use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashSet;

fn main() {
    if let Ok(lines) = read_lines("../day3a/input") {
        let mut pri_sum: u64 = 0;
        let mut count: u64 = 0;

        let mut shared: Vec<char> = Vec::new();
        let mut rs1 = HashSet::new();

        for line in lines {
            if let Ok(l) = line {
                //let mut shared: char = 'a';

                if count % 3 == 0 {
                    for c in l.chars() {
                        rs1.insert(c);
                    }
                }
                if count % 3 == 1 {
                    for c in l.chars() {
                        if rs1.contains(&c) {
                            shared.push(c);
                        }
                    }
                }
                if count % 3 == 2 {
                    let mut rs3 = HashSet::new();
                    let mut true_shared: char = 'a';
                    for c in l.chars() {
                        rs3.insert(c);
                    }
                    for s in shared.iter() {
                        if rs3.contains(&s) {
                            true_shared = *s;
                        }
                    }
                    let pri: u64;
                    if true_shared as i32 - 96 < 0 {
                        pri = true_shared as u64 - 64 + 26;
                    } else {
                        pri = true_shared as u64 - 96;
                    }

                    pri_sum += pri;
                    shared = Vec::new();
                    rs1 = HashSet::new();
                }

                count += 1;
            }
        }
        println!("{:?}", pri_sum);
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
