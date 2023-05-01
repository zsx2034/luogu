fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter =
        buf.split_whitespace().map(|s: &str| s.parse::<usize>().unwrap());
    let n = iter.next().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let sa: Vec<usize> = buf.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let sb: Vec<usize> = buf.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

    let template = [
        [0, -1, 1, 1, -1],
        [1, 0, -1, 1, -1],
        [-1, 1, 0, -1, 1],
        [-1, -1, 1, 0, 1],
        [1, 1, -1, -1, 0]
    ];

    let mut a = 0;
    let mut b = 0;
    for i in 0..n {
        if template[sa[i % sa.len()]][sb[i % sb.len()]] == 1 {
            a += 1;
        }
        if template[sb[i % sb.len()]][sa[i % sa.len()]] == 1 {
            b += 1;
        }
    }

    println!("{} {}", a, b);
}
