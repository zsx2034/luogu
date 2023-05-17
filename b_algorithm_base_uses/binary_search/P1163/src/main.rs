fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let total: f64 = iter.next().unwrap().parse().unwrap();
    let part_month: f64 = iter.next().unwrap().parse().unwrap();
    let m: u32 = iter.next().unwrap().parse().unwrap();

    let mut l = 0.0;
    let mut r = 10.0;
    while r - l > 1e-6 {
        let mid = (l + r) / 2.0;
        if check(m, mid, part_month, total) {
            r = mid;
        } else {
            l = mid;
        }
    }

    println!("{:.1}", l * 100.0);
}

fn check(m: u32, x: f64, part_month: f64, mut total: f64) -> bool{
    for _ in 0..m {
        total = total * (1.0 + x) - part_month;
    }
    if total < 0.0 {
        return false;
    }
    return true;
}
