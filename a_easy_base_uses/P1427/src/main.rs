fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let mut data: Vec<i32> = buf.split_whitespace().map(|num| num.parse().unwrap()).collect();

    data.pop();
    for num in data.iter().rev() {
        print!("{} ", num);
    }
}
