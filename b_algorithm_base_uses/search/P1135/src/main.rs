use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let A: usize = iter.next().unwrap().parse().unwrap();
    let A = A - 1;
    let B: usize = iter.next().unwrap().parse().unwrap();
    let B = B - 1;

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<usize> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut pos: VecDeque<usize> = VecDeque::from(vec![A]);
    let mut map = vec![0; n];

    while let Some(p) = pos.pop_front() {
        if p == B {
            println!("{}", map[B]);
            return;
        }

        if p + v[p] < n && map[p + v[p]] == 0 {
            map[p + v[p]] = map[p] + 1;
            pos.push_back(p + v[p]);
        }
        if p >= v[p] && map[p - v[p]] == 0 {
            map[p - v[p]] = map[p] + 1;
            pos.push_back(p - v[p]);
        }
    }

    println!("-1");
}
