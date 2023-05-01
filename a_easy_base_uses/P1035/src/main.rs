fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let num = buf.trim().parse::<f64>().unwrap();

    let mut sum = 0.0;
    let mut n = 1.0;
    while sum <= num {
        sum += 1.0 / n;
        n += 1.0;
    }

    println!("{}", n - 1.0);
}
