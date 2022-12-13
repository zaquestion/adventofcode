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
    /*
    match l {
        Value::Number(ln) => {
            match r {
                Value::Number(rn) => {
                    let b = ln.as_u64().expect("must num") <= rn.as_u64().expect("must num");
                    b
                },
                Value::Array(rarr) => {
                    let re = rarr.get(0);
                    if re == None {
                        return false
                    }
                    parse_packet(l, re.unwrap())
                },
                _ => false,
            }
        },
        Value::Array(larr) => {
            match r {
                Value::Number(_rn) => {
                    let le = larr.get(0);
                    if le == None {
                        return true
                    }
                    parse_packet(le.unwrap(), r)
                },
                Value::Array(rarr) => {
                    let mut in_order = true;
                    for (i, le) in larr.iter().enumerate() {
                        let re = rarr.get(i);
                        if re == None {
                            return false
                        }

                        in_order = parse_packet(le, re.unwrap());
                        if !in_order {
                            return false
                        }
                    };
                    return in_order;
                },
                _ => false,
            }
        },
        _ => true ,
    }
    */
    /*
    match l {
        Value::Number(n) => {
        },
        Value::Array(arr) => (),
        _ => (),
    }
    */

}

/*
fn parse_packet(el1: String, el2: String, depth: usize) -> bool {
    let mut llist: Vec<usize> = Vec::new();
    let mut rlist: Vec<usize> = Vec::new();

    let mut lnested: bool = false;
    let mut rnested: bool = false;
    if el1.len() == 0 && el2.len() == 0  {
        return true
    } else if el1.len() == 0 {
        return true
    } else if el2.len() == 0 {
        return false
    }
    let l = el1.chars().nth(0).unwrap();
    let r = el2.chars().nth(0).unwrap();
    /*
    println!("{:?} -- {:?}", el1, el2);
    el1.chars().take_while(|c| c != ']').

    if el1.chars().nth(0).unwrap() == '[' {
        lnested = el1.chars().nth(1).unwrap() == '[';
        if !lnested {
            llist =  el1.strip_prefix('[').unwrap().strip_suffix(']').unwrap().split(',') //.map(|c| c.parse::<usize>().expect("must num")).collect();
        }
    } else {
        llist.push(el1.parse::<usize>().expect("el2 must num"));
    }
    if el2.chars().nth(0).unwrap() == '[' {
        rnested = el2.strip_prefix('[').unwrap().chars().any(|c| c == '[');
        if !rnested {
            rlist =  el2.strip_prefix('[').unwrap().strip_suffix(']').unwrap().split(',').map(|c| c.parse::<usize>().expect("must num")).collect();
        }
    } else {
        rlist.push(el2.parse::<usize>().expect("el2 must num"));
    }
    */

    println!("l {:?}", el1);
    println!("r {:?}", el2);
    if l == ',' && r == ',' {
        let lnew = el1.strip_prefix(',').unwrap();
        let rnew = el2.strip_prefix(',').unwrap();
        return parse_packet(lnew.to_string(), rnew.to_string(), depth + 1);
    } else if l == ',' {
    } else if r == ',' {
    }


    let mut ordered = false;
    if l == '[' && r == '[' {
        let lnew = el1.strip_prefix('[').unwrap();
        let rnew = el2.strip_prefix('[').unwrap();
        return parse_packet(lnew.to_string(), rnew.to_string(), depth + 1);
    } else if l == '[' {
        let lnew = el1.strip_prefix('[').unwrap();
        if r == ']' {
            return false; // r ended early
        }
        let rnum = r.to_string().parse::<usize>().unwrap();
        return parse_packet(lnew.to_string(), format!("[{}]", rnum), depth + 1);

    } else if r == '[' {
        let rnew = el2.strip_prefix('[').unwrap();
        if l == ']' {
            return true; // l ended early
        }
        let lnum = l.to_string().parse::<usize>().unwrap();
        return parse_packet(format!("[{}]", lnum), rnew.to_string(), depth + 1);
    } else {
        if l == ']' {
            return true; // l ended early
        }
        if r == ']' {
            return false; // r ended early
        }

        let lnum = l.to_string().parse::<usize>().unwrap();
        let rnum = r.to_string().parse::<usize>().unwrap();

        ordered = lnum <= rnum;
        if !ordered {
            return false
        } else {
            let lnew: String = el1.chars().into_iter().skip(2).collect();
            let rnew: String = el2.chars().into_iter().skip(2).collect();
            return parse_packet(lnew, rnew, depth + 1);
        }
    }
    /*
    if lnested && !rnested {
        println!("r nested");
        let lnew = el1.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
        let rnum = el2.parse::<usize>().expect("el2 must num");
        return parse_packet(lnew.to_string(), format!("[{}]", rnum));
    }
    if !lnested && rnested {
        println!("l nested");
        let lnum = el1.parse::<usize>().expect("el1 must num");
        let rnew = el2.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
        return parse_packet(format!("[{}]", lnum), rnew.to_string());
    }

    for (i, num) in llist.iter().enumerate() {
        println!("{:?} -- {:?}", num, rlist[i]);
        if *num > rlist[i] {
            return false
        }
    };
    return true
    */
    return false
}
*/

fn part2(lines: &Vec<String>) {
    let mut packets: Vec<serde_json::Value> = Vec::new();
    lines.chunks(3).into_iter().for_each(|chunk| {
        let c = chunk.clone();
        //let el1 = &c[0];
        //let el2 = &c[1];

        //let in_order = parse_packet(el1.to_string(), el2.to_string(), 0);
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
