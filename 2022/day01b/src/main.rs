use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut elf_cals: Vec<i64> = Vec::new();
    let mut sum: i64 = 0;
    if let Ok(lines) = read_lines("../day1a/input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(kcals) = line {
                sum += kcals.trim_end().parse::<i64>().unwrap_or_default();
                if kcals == String::from("") {
                    elf_cals.push(sum);
                    sum = 0;
                }
            }
        }
    }
    elf_cals.sort();
    println!("{}", elf_cals.iter().rev().take(3).sum::<i64>())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
