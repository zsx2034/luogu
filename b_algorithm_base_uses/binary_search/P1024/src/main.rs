fn main() {
    // resolve ax^3 + bx^2 + cx + d = 0
    // it has threee different roots
    // so 3ax^2 + 2bx + c = 0 has two different roots
    // so 6ax + 2b = 0 has one root

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let a: f64 = iter.next().unwrap().parse().unwrap();
    let b: f64 = iter.next().unwrap().parse().unwrap();
    let c: f64 = iter.next().unwrap().parse().unwrap();
    let d: f64 = iter.next().unwrap().parse().unwrap();

    // the two ans of the 3ax^2 + 2bx + c = 0
    let  ans1 = (-2.0*b - (4.0*b.powi(2) - 12.0*a*c).sqrt()) / (6.0*a);
    let  ans2 = (-2.0*b + (4.0*b.powi(2) - 12.0*a*c).sqrt()) / (6.0*a);

    // we can get three areas by the two ans
    let area = vec![(-100.0, ans1), (ans1, ans2), (ans2, 100.0)];
    let ans = binary_search(-100.0, ans1, a, b, c, d);
    print!("{:.2} ", ans);

    let ans = binary_search_recv(ans1, ans2, a, b, c, d);
    print!("{:.2} ", ans);

    let ans = binary_search(ans2, 100.0, a, b, c, d);
    print!("{:.2}", ans);
}

fn binary_search(l: f64, r: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    let mut l = l;
    let mut r = r;
    while r - l > 1e-9 {
        let mid = (l + r) / 2.0;
        let tmp = calc(mid, a, b, c, d);
        if  tmp > 0.0 {
            r = mid;
        } else {
            l = mid;
        }
    }
    l
}

fn binary_search_recv(l: f64, r: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    let mut l = l;
    let mut r = r;
    while r - l > 1e-9 {
        let mid = (l + r) / 2.0;
        let tmp = - calc(mid, a, b, c, d);
        if  tmp > 0.0 {
            r = mid;
        } else {
            l = mid;
        }
    }
    l
}

fn calc(x: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * x.powi(3) + b * x.powi(2) + c * x + d
}