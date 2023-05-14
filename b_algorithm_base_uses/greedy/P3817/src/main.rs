fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: i64 = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut v: Vec<i64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut ans = 0;
    for i in 0..n-1 {
        if v[i] + v[i+1] > m {
            let tmp = v[i] + v[i+1] - m;
            ans += tmp;
            if v[i+1] >= tmp {
                v[i+1] -= tmp;
            } else {
                v[i] -= tmp - v[i+1];
                v[i+1] = 0;
            }
        }
    }

    println!("{}", ans);
}
