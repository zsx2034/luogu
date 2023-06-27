use std::{cmp::Ordering, collections::VecDeque};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut i = 0;
    let mut v: VecDeque<(i32, i32)> = buf
        .split_whitespace()
        .map(|x| {
            i += 1;
            (x.parse().unwrap(), i)
        })
        .collect();

    while v.len() > 2 {
        let a = v.pop_front().unwrap();
        let b = v.pop_front().unwrap();
        match a.0.cmp(&b.0) {
            Ordering::Less => v.push_back(b),
            Ordering::Greater => v.push_back(a),
            Ordering::Equal => (),
        }
    }

    println!("{}", if v[0].0 < v[1].0 { v[0].1 } else { v[1].1 });
}