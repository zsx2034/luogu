
fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let upbound: u64 = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut v: Vec<u64> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        v.push(buf.trim().parse().unwrap());
    }

    v.sort();
    let mut ans = 0;
    let mut i = 0;
    let mut j  = n - 1;

    while i <= j {
        if v[i] + v[j] <= upbound {
            i += 1;
        }
        j -= 1;
        ans += 1;
    }

    println!("{}", ans);
}
