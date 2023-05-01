fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut m = (m as f64 * 1.5) as usize;

    let mut candidates: Vec<(u64, u64)> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let id: u64 = iter.next().unwrap().parse().unwrap();
        let score: u64 = iter.next().unwrap().parse().unwrap();
        candidates.push((id, score));
    }

    candidates.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    let score = candidates[m - 1].1;
    while m < n {
        if candidates[m].1 != score {
            break;
        }
        m += 1;
    }

    println!("{} {}", score, m);
    for i in 0..m {
        println!("{} {}", candidates[i].0, candidates[i].1);
    }

}
