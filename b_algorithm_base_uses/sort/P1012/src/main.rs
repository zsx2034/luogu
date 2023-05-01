fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut a: Vec<Vec<char>> = buf
        .split_whitespace()
        .map(|s| s.chars().collect())
        .collect();

    a.sort_by(
        |x: &Vec<char>, y: &Vec<char>| {
            let short_len = std::cmp::min(x.len(), y.len());

            for i in 0..short_len {
                if x[i] != y[i] {
                    return y[i].cmp(&x[i]);
                }
            }

            if x.len() == y.len() {
                return std::cmp::Ordering::Equal;
            }
            
            if y.len() > x.len() {
                return y[short_len].cmp(&x[0]);
            } else {
                return y[0].cmp(&x[short_len]);
            }

        }
    );

    for i in 0..n {
        print!("{}", a[i].iter().collect::<String>());
    }
}
