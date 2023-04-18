fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut cnt = 0;
    let sum = buf
        .split_whitespace()
        .map(|x| {
            cnt += 1;
            x.parse::<i128>().unwrap()
        })
        .sum::<i128>();
    let times = 2_i128.pow(cnt - 1);
    println!("{}", sum * times);
}
