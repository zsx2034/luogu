fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut id = 0;
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut v: Vec<Person> = buf
        .split_whitespace()
        .map(|x| {
            id += 1;
            Person {
                t: x.trim().parse().unwrap(),
                id: id
            }
        })
        .collect();
    v.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

    let n = n as f64;

    let mut ans = 0.0;
    for (i , x) in v.iter().enumerate() {
        ans += x.t * (n - i as f64 - 1.0)/ n;
        print!("{} ", x.id);
    }
    println!();
    println!("{:.2}", ans);
}

#[derive(Debug)]
struct Person {
    t: f64,
    id: u32
}