fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: i64 = buf.trim().parse().unwrap();

    // F[n] = F[n-1] + F[n-2] + 2*G[n-2]
    // G[n] = F[n-1] + G[n-1]
    let mut f = vec![0; n as usize + 1];
    let mut g = vec![0; n as usize + 1];

    f[0] = 1;
    g[0] = 0;
    f[1] = 1;
    g[1] = 1;

    for i in 2..=n as usize {
        f[i] = (f[i - 1] + f[i - 2] + 2 * g[i - 2]) % 10000;
        g[i] = (f[i - 1] + g[i - 1]) % 10000;
    }

    println!("{}", f[n as usize]);
}
