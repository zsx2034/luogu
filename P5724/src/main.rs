fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let mut n = buf.trim().parse::<usize>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let mut data: Vec<_> = buf.split_whitespace().map(|nums| nums.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    data.sort();

    println!("{}", (data[n - 1] - data[0]));
}
