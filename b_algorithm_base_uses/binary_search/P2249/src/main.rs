fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut v: Vec<(usize, u64)> = Vec::with_capacity(n);
    for (i, num) in buf.split_whitespace().map(|x| x.parse().unwrap()).enumerate() {
        if v.is_empty() || v.last().unwrap().1 != num {
            v.push((i+1, num));
        }
    }

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    for num in buf.split_whitespace().map(|x| x.parse().unwrap()) {
        match search(&v, num) {
            Some(i) => {
                print!("{} ", i);
            }
            None => {
                print!("-1 ");
            }
        }
    }
}

fn search(v: &Vec<(usize, u64)>, t: u64) -> Option<usize> {
    let mut l = 0;
    let mut r = v.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        match v[m].1.cmp(&t) {
            std::cmp::Ordering::Less => l = m + 1,
            std::cmp::Ordering::Equal => return Some(v[m].0),
            std::cmp::Ordering::Greater => {
                if r == 0 {
                    return None;
                }
                r = m - 1;
            }
        }
    }
    None
}
