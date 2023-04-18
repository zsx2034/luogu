fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let s = buf.trim().parse::<f64>().unwrap();

    let mut sum = 0.0;
    let mut v = 2.0;
    let mut cnt = 0;
    while sum < s {
        sum += v;
        cnt += 1;
        v *= 0.98;
    }

    print!("{}", cnt);
}
