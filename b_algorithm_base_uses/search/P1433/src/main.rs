use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut v: Vec<(f64, f64)> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let x: f64 = iter.next().unwrap().parse().unwrap();
        let y: f64 = iter.next().unwrap().parse().unwrap();
        v.push((x, y));
    }

    let mut v: VecDeque<(f64, f64)> = VecDeque::from(v);
    v.push_front((0.0, 0.0));

    let mut distance_map = vec![vec![0.0; n + 1]; n + 1];
    for i in 0..v.len() {
        for j in i..v.len() {
            distance_map[i][j] = distance(v[i], v[j]);
            distance_map[j][i] = distance_map[i][j];
        }
    }

    // 1 << (n + 1) == 2 ^ (n + 1)
    let mut f = vec![vec![f64::MAX; n + 1 ]; 1 << (n)];

    f[0][0] = 0.0;
    for i in 1..n + 1 {
        f[1 << (i - 1)][i] = distance_map[0][i];
    }

    for k in 1..1 << n {
        for i in 1..n + 1 {
            // 判断第 i 位是否为 1，即点i是否已经被访问过
            if k & (1 << (i - 1)) == 0 {
                continue;
            }

            for j in 1..n + 1 {
                if i == j {
                    continue;
                }
                if k & (1 << (j - 1)) == 0 {
                    continue;
                }

                let new = f[k - (1 << (i - 1))][j] + distance_map[j][i];
                if f[k][i] > new {
                    f[k][i] = new;
                }
            }
        }
    }

    let mut ans = f64::MAX;
    let k = (1 << n) - 1;
    for i in 1..n + 1 {
        if ans > f[k][i] {
            ans = f[k][i];
        }
    }

    println!("{:.2}", ans);
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let x = a.0 - b.0;
    let y = a.1 - b.1;
    (x * x + y * y).sqrt()
}
