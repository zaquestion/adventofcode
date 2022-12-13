use std::io;
use serde_json::Value;
use std::cmp::Ordering;

fn part1(lines: &Vec<String>) {
    let mut dexes: Vec<usize> = Vec::new();
    let mut pair = 1;
    lines.chunks(3).into_iter().for_each(|chunk| {
        let c = chunk.clone();
        let l: serde_json::Value = serde_json::from_str(&c[0]).unwrap();
        let r: serde_json::Value = serde_json::from_str(&c[1]).unwrap();

        if cmp_packet(&l, &r) == Ordering::Less {
            dexes.push(pair);
        }

        pair += 1;
    });
    println!("part1: {}", dexes.iter().sum::<usize>() );
}

fn cmp_packet(l: &serde_json::Value, r: &serde_json::Value) -> Ordering {
    //println!("Next l: {:?}\nNext r: {:?}", l, r);
    if let (Value::Number(ln), Value::Number(rn)) = (l,r) {
        if ln.as_u64().unwrap() < rn.as_u64().unwrap() {
            return Ordering::Less
        } else if ln.as_u64().unwrap() == rn.as_u64().unwrap() {
            return Ordering::Equal
        } else {
            return Ordering::Greater
        }
    }

    let le = match l {
        Value::Number(_ln) => vec![l.clone()],
        Value::Array(larr) => larr.to_vec(),
        _ => vec![serde_json::json!([])],
    };
    let re = match r {
        Value::Number(_rn) => vec![r.clone()],
        Value::Array(rarr) => rarr.to_vec(),
        _ => vec![serde_json::json!([])],
    };

    for (i, le) in le.iter().enumerate() {
        let re = re.get(i);
        if re == None {
            return Ordering::Greater
        }

        let res = cmp_packet(le, re.unwrap());
        if res != Ordering::Equal {
            return res
        }
    };
    if le.len() < re.len() {
        return Ordering::Less
    }
    Ordering::Equal
}

fn part2(lines: &Vec<String>) {
    let mut packets: Vec<serde_json::Value> = Vec::new();
    lines.chunks(3).into_iter().for_each(|chunk| {
        let c = chunk.clone();
        let l: serde_json::Value = serde_json::from_str(&c[0]).unwrap();
        let r: serde_json::Value = serde_json::from_str(&c[1]).unwrap();

        packets.push(l);
        packets.push(r);
    });
    let div1: Value = serde_json::from_str("[[2]]").unwrap();
    let div2: Value = serde_json::from_str("[[6]]").unwrap();
    packets.push(div1.clone());
    packets.push(div2.clone());
    packets.sort_by(|p1,p2| cmp_packet(p1, p2));

    let d1i = packets.iter().position(|p| *p == div1).unwrap() + 1;
    let d2i = packets.iter().position(|p| *p == div2).unwrap() + 1;
    println!("part2: {}", d1i*d2i)
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
                                .filter_map(Result::ok).collect();
    //let input = io::read_to_string(io::stdin()).unwrap();

    part1(&lines);
    part2(&lines);
}
