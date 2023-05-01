fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let range: Vec<usize> = buf
        .trim()
        .split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut res = vec![0; 10];
    let start = range[0];
    let end = range[1];
    for mut num in start..end + 1 {
        while num > 0 {
            res[(num % 10)] += 1;
            num /= 10;
        }
    }

    for num in res {
        print!("{} ", num);
    }
}
