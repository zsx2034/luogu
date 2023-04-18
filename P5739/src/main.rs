fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    println!("{}", calc(n));
}

fn calc(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    calc(n - 1) * n
}