fn main() {
    // let mut buf = String::new();
    // std::io::stdin().read_line(&mut buf).unwrap();
    //
    // buf.clear();
    // std::io::stdin().read_line(&mut buf).unwrap();
    //
    // let mut v = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    //
    // v.sort_unstable();
    //
    // for v in v.iter() {
    //     print!("{} ", v);
    // }

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let k = iter.next().unwrap().parse::<usize>().unwrap();

    let mut r = vec![0; n + 1];

    buf.clear();
    buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).enumerate().for_each(|(i, x)| r[x] += 1);

    for i in 0..n + 1 {
        for _ in 0..r[i] {
            print!("{} ", i);
        }
    }
}
