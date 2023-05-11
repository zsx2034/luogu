fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut buf = buf.trim();

    let mut s = String::new();
    let mut n: u128 = 0;
    if buf.contains(' ') {
        let mut iter = buf.split_whitespace();
        s = iter.next().unwrap().to_string();
        n = iter.next().unwrap().parse().unwrap();
    } else {
        s = buf.chars().collect();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        n = buf.trim().parse().unwrap();
    }

    let s: Vec<char> = s.chars().collect();
    let upbound = ((n as f64) / (s.len() as f64)).log2().ceil() as u128;
    let mut upbound = 2u128.pow(upbound as u32) * s.len() as u128;

    let slen = s.len() as u128;
    while upbound > slen {
        upbound /= 2;
        if n <= upbound {
            continue;
        } else {
            n -= upbound;
        }
        if n <= 1 {
            n += upbound - 1;
        } else {
            n -= 1;
        }
    }

    if n < 1 {
        n += slen;
    } 
    println!("{}", s[(n - 1) as usize]);
}
