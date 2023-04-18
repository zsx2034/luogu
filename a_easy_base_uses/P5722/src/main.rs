fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n: i64 = buf.trim().parse::<i64>().unwrap() + 1;

    let mut sum = 0;
    for i in 1..n {
        sum += i;
    }

    println!("{}", sum);
}