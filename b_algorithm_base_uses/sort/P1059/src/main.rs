use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<usize> = buf
        .split_whitespace()
        .map(|x| x.parse::<_>().unwrap())
        .collect();

    let mut r = vec![(0, false); 1001];

    for num in v {
        r[num] = (num, true);
    }

    let mut iter = r.iter().filter(|x| x.1);
    let mut tmp = iter.clone();

    println!("{}", iter.count());
    tmp.for_each(|x| print!("{} ", x.0));
}
