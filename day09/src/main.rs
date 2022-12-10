use std::io;

fn part1(lines: &Vec<String>) {
    let mut head_motions = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let direction = parts[0];
        let steps = parts[1].parse::<usize>().unwrap();
        head_motions.push((direction, steps));
    }

    // Initialize the head and tail positions
    let mut head_position: (i64, i64) = (0, 0);
    let mut tail_position: (i64, i64) = (0, 0);
    let mut tail_history: Vec<(i64, i64)> = Vec::new();

    // Follow the head's motions and update the tail's position
    for (direction, steps) in head_motions {
        // Move the head in the given direction
        for _ in 0..steps {
            match direction {
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                "L" => head_position.0 -= 1,
                "R" => head_position.0 += 1,
                _ => panic!("Invalid direction: {}", direction),
            }

            // Check if the head and tail are touching
            let is_touching = (head_position.0 == tail_position.0
                && (head_position.1 == tail_position.1 + 1
                    || head_position.1 == tail_position.1 - 1))
                || (head_position.1 == tail_position.1
                    && (head_position.0 == tail_position.0 + 1
                        || head_position.0 == tail_position.0 - 1))
                || (head_position.0 == tail_position.0 && head_position.1 == tail_position.1)
                || (head_position.1 == tail_position.1 + 1
                    && (head_position.0 == tail_position.0 + 1
                        || head_position.0 == tail_position.0 - 1))
                || (head_position.1 == tail_position.1 - 1
                    && (head_position.0 == tail_position.0 + 1
                        || head_position.0 == tail_position.0 - 1));

            // Update the tail's position if necessary
            if is_touching {
                continue
            }

            if head_position.0 - tail_position.0 >= 1 {
                tail_position.0 += 1;
            } else if tail_position.0 - head_position.0 >= 1 {
                tail_position.0 -= 1;
            }
            if head_position.1 - tail_position.1 >= 1 {
                tail_position.1 += 1;
            } else if tail_position.1 - head_position.1 >= 1 {
                tail_position.1 -= 1;
            }

            tail_history.push(tail_position);
        }
    }

    tail_history.sort();
    tail_history.dedup();

    println!("{}", tail_history.len());
}

fn part2(lines: &Vec<String>) {
    let mut head_motions = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let direction = parts[0];
        let steps = parts[1].parse::<usize>().unwrap();
        head_motions.push((direction, steps));
    }

    // Initialize the head and tail positions
    let mut head_position: (i64, i64) = (0, 0);
    let mut knots: [(i64, i64); 10] = [(0, 0); 10];

    let mut visited_positions = std::collections::HashSet::new();

    // Follow the head's motions and update the tail's position
    for (direction, steps) in head_motions {
        // Move the head in the given direction
        for _ in 0..steps {
            match direction {
                "U" => head_position.1 += 1,
                "D" => head_position.1 -= 1,
                "L" => head_position.0 -= 1,
                "R" => head_position.0 += 1,
                _ => panic!("Invalid direction: {}", direction),
            }
            knots[0] = head_position;
            for i in 1..10 {
                let prev_knot = knots[i - 1];
                let cur_knot = knots[i];

                let is_touching = (prev_knot.0 == cur_knot.0
                    && (prev_knot.1 == cur_knot.1 + 1 || prev_knot.1 == cur_knot.1 - 1))
                    || (prev_knot.1 == cur_knot.1
                        && (prev_knot.0 == cur_knot.0 + 1 || prev_knot.0 == cur_knot.0 - 1))
                    || (prev_knot.1 == cur_knot.1 + 1
                        && (prev_knot.0 == cur_knot.0 + 1 || prev_knot.0 == cur_knot.0 - 1))
                    || (prev_knot.1 == cur_knot.1 - 1
                        && (prev_knot.0 == cur_knot.0 + 1 || prev_knot.0 == cur_knot.0 - 1))
                    || (prev_knot.0 == cur_knot.0 && prev_knot.1 == cur_knot.1);

                if is_touching {
                    continue
                }

                if prev_knot.0 - cur_knot.0 >= 1 {
                    knots[i].0 += 1;
                } else if cur_knot.0 - prev_knot.0 >= 1 {
                    knots[i].0 -= 1;
                }
                if prev_knot.1 - cur_knot.1 >= 1 {
                    knots[i].1 += 1;
                } else if cur_knot.1 - prev_knot.1 >= 1 {
                    knots[i].1 -= 1;
                }
            }

            visited_positions.insert(knots[9]);
        }
        /*
        println!("{},{}", direction, steps);
        for y in (-5..17).rev() {
            for x in -11..20 {
                let mut found = false;
                for i in 0..10 {
                    let knot = knots[i];
                    if knot.0 == x && knot.1 == y {
                        if i == 0 {
                            print!("H");
                        } else {
                            print!("{}", i);
                        }
                        found = true;
                        break;
                    }
                }
                if !found && x == 0 && y == 0 {
                    print!("s");
                } else if !found {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
        */
    }

    println!("{}", visited_positions.len());
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    part1(&lines);
    part2(&lines);
}
