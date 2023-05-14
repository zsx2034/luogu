fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut a: Vec<u128> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    a.sort();

    // init with the situation -- jump from floor to the highest floor 
    let mut i = 0;
    let mut j = n - 1;
    let mut ans = a[j].pow(2);
    let mut flag = true;
    while i < j {
        ans += (a[j] - a[i]).pow(2);
        if flag {
            j -= 1;
        } else {
            i += 1;
        }
        flag = !flag;
    }

    println!("{}", ans);
}
