use std::collections::VecDeque;

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

    let mut v = VecDeque::from(v);
    let mut ans = 0;
    while v.len() > 1 {
        let a = v.pop_front().unwrap();
        let b = upbound - a;

        for i in (0..v.len()).rev() {
            if v[i] <= b {
                v.remove(i);
                break;
            }
        }

        ans += 1;
    }

    if v.len() == 1 {
        ans += 1;
    }

    println!("{}", ans);
}
