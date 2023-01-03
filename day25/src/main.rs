use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().filter_map(Result::ok).collect();
    let snafus = lines
        .iter()
        .map(|l| snafu_to_dec(l.to_string()))
        .collect::<Vec<isize>>();
    for i in 0..snafus.len() {
        println!("{:?} -- {:?}", lines[i], snafus[i]);
    }

    let snasum = snafus.iter().sum::<isize>();
    println!("sum in snafu: {:?}", dec_to_snafu(snasum));
    // WRONG: 2=0--21==-0--2201--2

    /*
    let n = 1257;
    println!("num: {}, snafu {:?}", n, dec_to_snafu(n));
    let n = 37;
    println!("num: {}, snafu {:?}", n, dec_to_snafu(n));
    */
}

fn dec_to_snafu(n: isize) -> String {
    /*
     * 1
     * 2
     * 1=
     * 1-
     * 10
     *
     * 11
     * 12
     * 2=
     * 2-
     * 20
     */
    /*
    let q = n / 5;
    let q = n % 5;
    */
    /*
    let pows = (0..10)
        .map(|n| 5isize.pow(n as u32))
        .collect::<Vec<isize>>();

    println!("pows: {:?}", pows);
    */
    let mut num = n;
    let mut str_parts = Vec::new();

    while num > 0 {
        let r = num % 5;
        str_parts.insert(
            0,
            match r {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "=",
                4 => "-",
                _ => todo!("bad modulo"),
            },
        );
        dbg!(num, r);
        if r >= 3 {
            num += 5;
        }
        num /= 5;
    }

    str_parts.join("")
}

fn snafu_to_dec(s: String) -> isize {
    s.chars().rev().enumerate().fold(0isize, |sum, (i, c)| {
        sum + (match c {
            '2' => 2isize,
            '1' => 1isize,
            '0' => 0,
            '-' => -1isize,
            '=' => -2isize,
            _ => todo!("you fucked something up in your parser"),
        }) * 5isize.pow(i as u32)
    })
}
