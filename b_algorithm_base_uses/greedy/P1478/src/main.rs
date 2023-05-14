fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut s: i64 = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let a: u64 = iter.next().unwrap().parse().unwrap();
    let b: u64 = iter.next().unwrap().parse().unwrap();

    let mut v: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let h_: u64 = iter.next().unwrap().parse().unwrap();
        let s_: i64 = iter.next().unwrap().parse().unwrap();
        if h_ > b + a {
            continue;
        }
        v.push(s_);
    }

    v.sort();

    let mut ans = 0;
    for s_ in v {
        if s_ > s {
            break;
        }
        s -= s_;
        ans += 1;
    }

    println!("{}", ans);
}
