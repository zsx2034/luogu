fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut v = Vec::with_capacity(n);
    let mut min = i32::MAX;
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.trim().split_whitespace();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();

        min = std::cmp::min(min, (a - b).abs());

        v.push((a, b));
    }

    dp(&v, 0, 1, 0, &mut min);

    println!("{}", min);
}

fn dp(v: &Vec<(i32, i32)>, i: usize, sum1: i32, sum2: i32, min: &mut i32) {
    if i == v.len() {
        if (sum1 != 1 && sum2 != 0) {
            *min = std::cmp::min(*min, (sum1 - sum2).abs());
        }
    } else {
        dp(v, i + 1, sum1 * v[i].0, sum2 + v[i].1, min);
        dp(v, i + 1, sum1, sum2, min);
    }
}
