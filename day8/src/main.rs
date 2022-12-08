use std::io;

fn count_visible_trees(tree_heights: &Vec<Vec<u8>>) -> u32 {
    let mut count = 0;

    for r in 1..tree_heights.len()-1 {
        for j in 1..tree_heights[r].len()-1 {
            let height = tree_heights[r][j];

            let mut left = true;
            for k in 0..j {
                if tree_heights[r][k] >= height {
                    left = false;
                    break;
                }
            }

            let mut right = true;
            for k in j+1..tree_heights[r].len() {
                if tree_heights[r][k] >= height {
                    right = false;
                    break;
                }
            }

            let mut top = true;
            for k in 0..r {
                if tree_heights[k][j] >= height {
                    top = false;
                    break;
                }
            }

            let mut bottom = true;
            for k in r+1..tree_heights.len() {
                if tree_heights[k][j] >= height {
                    bottom = false;
                    break;
                }
            }

            if left || top || bottom || right {
                count += 1;
            }
        }
    }

    count += tree_heights.first().unwrap().len() * 2;
    count += tree_heights.len() * 2;
    count -= 4; // acount for corner overlap
    count as u32
}

fn count_scenic(tree_heights: &Vec<Vec<u8>>) -> u32 {
    let mut max = 0;
    for r in 0..tree_heights.len() {
        for j in 0..tree_heights[r].len() {
            let height = tree_heights[r][j];

            let mut left = 0;
                for k in (0..j).rev() {
                    left += 1;
                    if tree_heights[r][k] >= height {
                        break;
                    }
                }

             let mut right = 0;
                for k in j+1..tree_heights[r].len() {
                    right += 1;
                    if tree_heights[r][k] >= height {
                        break;
                    }
                }

            let mut up = 0 ;
                for k in (0..r).rev() {
                    up += 1;
                    if tree_heights[k][j] >= height {
                        break;
                    }
                }

            let mut down = 0;
                for k in r+1..tree_heights.len() {
                    down += 1;
                    if tree_heights[k][j] >= height {
                        break;
                    }
                }
            let count = left * right * up *down;
            if count > max {
                max = count;
            }
        }
    }
    max as u32
}

fn convert_strings_to_bytes(strings: &Vec<String>) -> Vec<Vec<u8>> {
    let mut bytes: Vec<Vec<u8>> = vec![];

    for string in strings {
        let mut byte_vec: Vec<u8> = vec![];
        for ch in string.chars() {
            byte_vec.push(ch as u8);
        }
        bytes.push(byte_vec);
    }

    bytes
}

fn part1(lines: &Vec<String>) {
    let bytes = convert_strings_to_bytes(lines);
    println!("{}", count_visible_trees(&bytes));

}

fn part2(lines: &Vec<String>) {
    let bytes = convert_strings_to_bytes(lines);
    println!("{}", count_scenic(&bytes));
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    part1(&lines);
    part2(&lines);
}
