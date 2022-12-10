use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum = 0;
        for line in lines {
            if let Ok(play_guide) = line {
                let parts = play_guide.split(" ").collect::<Vec<&str>>();
                let (l, r) = (parts[0], parts[1]);
                let mut hand_score = 0;
                match r {
                    "X" => hand_score = 1,
                    "Y" => hand_score = 2,
                    "Z" => hand_score = 3,
                    _ => (),
                };
                let mut play_score = 0;
                match l {
                    "A" => match r {
                        "X" => play_score = 3,
                        "Y" => play_score = 6,
                        "Z" => play_score = 0,
                        _ => (),
                    },
                    "B" => match r {
                        "X" => play_score = 0,
                        "Y" => play_score = 3,
                        "Z" => play_score = 6,
                        _ => (),
                    },
                    "C" => match r {
                        "X" => play_score = 6,
                        "Y" => play_score = 0,
                        "Z" => play_score = 3,
                        _ => (),
                    },
                    _ => (),
                };
                sum += hand_score+play_score;
            }
        }
        println!("{}", sum);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
