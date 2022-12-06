use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let (board, moves) = lines.split_at(9);

    for col in (0..=8) {
        let mut stack: Vec<char> = Vec::new();
        let mut h: usize = 7;
        loop {
            if board[h].chars().nth(col*4) == Some('[') {
                stack.push(board[h].chars().nth((col*4)+1).unwrap_or_default());
            } else {
                break;
            }
            if h == 0 {
                break;
            }
            h  -= 1;
        }
        stacks.push(stack);

    }

    for m  in moves.iter().skip(1) {
        let ms = m.split(' ').collect::<Vec<_>>();
        let n = ms[1].parse::<i32>().unwrap_or_default();
        let from = ms[3].parse::<usize>().unwrap_or_default();
        let to = ms[5].parse::<usize>().unwrap_or_default();

        for i in 1..=n {
            let b = stacks[from-1].pop().unwrap();
            stacks[to-1].push(b);
        }
    }
    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }

}
