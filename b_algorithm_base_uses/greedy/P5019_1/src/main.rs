fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut a: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut ans = a[0];
    for i in 1..n {
        if a[i] <= a[i - 1] {
            continue;
        }
        ans += a[i] - a[i - 1];
    }
    println!("{}", ans);
}
