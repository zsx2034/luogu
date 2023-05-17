fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let p: f64 = iter.next().unwrap().parse().unwrap();

    let mut v: Vec<(f64, f64)> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let x: f64 = iter.next().unwrap().parse().unwrap();
        let y: f64 = iter.next().unwrap().parse().unwrap();
        v.push((x, y));
    }

    let mut sum = 0.0;
    for item in v.iter() {
        sum += item.0;
    }

    if sum <= p {
        println!("-1");
        return;
    }

    let mut l = 0.0;
    let mut r = 1e11;

    while r - l > 1e-6 {
        let mid = (l + r) / 2.0;
        if check(&v, mid, p) {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", l);

}

fn check(v: &Vec<(f64, f64)>, t: f64, mut p: f64) -> bool {
    for &(x, y) in v {
        let tmp = (x * t - y) / t;
        if tmp > 0.0 {
            if p < tmp {
                return false;
            } else {
                p -= tmp;
            }
        }
    }
    true
}
