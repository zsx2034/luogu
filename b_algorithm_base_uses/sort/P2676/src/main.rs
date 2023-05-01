use std::{collections::{BinaryHeap}, cmp::Reverse};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let hmax: i64 = iter.next().unwrap().parse().unwrap();

    let mut h: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut sum = 0;
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let hi: i64 = buf.trim().parse().unwrap();
        h.push(Reverse(hi));
        sum += hi;
        while let Some(Reverse(hmin)) = h.peek() {
            if sum - hmin < hmax {
                break;
            }
            sum -= hmin;
            h.pop();
        }
    }

    println!("{}", h.len());
}
