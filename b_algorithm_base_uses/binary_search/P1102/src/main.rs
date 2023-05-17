use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let C: u64 = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut m = HashMap::with_capacity(n);
    for num in buf.split_whitespace().map(|x| x.parse::<u64>().unwrap()) {
        let count = m.entry(num).or_insert(1);
        *count += 1;
    }

    let mut ans = 0;
    for (k, v) in m.iter() {
        if k < &C {
            continue;
        }

        if let Some(&count) = m.get(&(k - C)) {
            ans += count * v;
        }
    }

    println!("{}", ans);

}

