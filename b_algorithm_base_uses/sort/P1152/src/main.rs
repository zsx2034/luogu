fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let v: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let mut record = vec![false; n];

    for i in 0..n-1 {
        let tmp = (v[i] - v[i + 1]).abs();
        if tmp >= n as i64 {
            println!("Not jolly");
            return;
        }
        record[tmp as usize] = true;
    }

    for i in 1..n {
        if !record[i] {
            println!("Not jolly");
            return;
        }
    }

    println!("Jolly");
}
