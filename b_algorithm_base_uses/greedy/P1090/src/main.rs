use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut a: BinaryHeap<Reverse<u64>> = buf
        .split_whitespace()
        .map(|x| Reverse(x.parse::<u64>().unwrap()))
        .collect();

    let mut sum = 0;
    while a.len() > 1 {
        let x = a.pop().unwrap().0;
        let y = a.pop().unwrap().0;
        sum += x + y;
        a.push(Reverse(x + y));
    }

    println!("{}", sum);
}
