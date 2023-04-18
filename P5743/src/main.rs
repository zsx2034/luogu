fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    let mut sum = 1;
    for _ in 0..n-1 {
        sum = (sum + 1) * 2;
    }
    println!("{}", sum);
}
