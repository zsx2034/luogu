fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let mut n: i32 = buf.trim().parse().unwrap();
    let mut record: Vec<i32> = Vec::new();
    record.push(n);

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        record.push(n);
    }

    for r in record.iter().rev() {
        print!("{} ", r);
    }
}
