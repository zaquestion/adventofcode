use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashSet;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        let mut pri_sum: u32 = 0;
        for line in lines {
            let mut rs1 = HashSet::new();

            if let Ok(l) = line {
                //let mut shared: char = 'a';
                let (r1, r2) = l.split_at(l.len()/2);
                let mut shared: char = 'a';
                for c in r1.chars() {
                    rs1.insert(c);
                }
                for c in r2.chars() {
                    if rs1.contains(&c) {
                        shared = c;
                    }
                }

                let pri: u32;
                if shared as i32 - 96 < 0 {
                    pri = shared as u32 - 64 + 26;
                } else {
                    pri = shared as u32 - 96;
                }

                println!("{} -- {} -- {} -- {}", r1, r2, shared, pri);

                pri_sum += pri
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
