fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<usize> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    for (i, ask) in buf.split_whitespace().map(|x| x.parse::<usize>().unwrap() - 1).enumerate() {
        println!("{}", v[ask]);
    }
}
