fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    let mut v = vec![1; n + 1];

    solve(&mut v, n);

    println!("{}", v[n]);
    // println!("{:?}", v);
}

fn solve(v: &mut Vec<u32>, n: usize) {
    if v[n] != 1 || n == 1{
        return;
    }

    for i in 1..=n / 2 {
        solve(v, i);
        v[n] += v[i];
    }


}
