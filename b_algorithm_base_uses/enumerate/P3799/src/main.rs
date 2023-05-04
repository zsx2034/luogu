const MOD: u64 = 10_0000_0007;

fn main() {
    let mut v = vec![0; 5001];

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        v[buf.trim().parse::<usize>().unwrap()] += 1;
    }

    let mut cnt: u64 = 0;
    for i in 1..=5000 {
        if v[i] <= 1 {
            continue;
        }

        let l = i;
        let mut tmp = 0;
        let mut upbound = 0;

        if l % 2 == 0 {
            if v[l / 2] > 1 {
                tmp += C2(v[l / 2]);
            }
            upbound = l / 2 - 1;
        } else {
            upbound = l / 2;
        }

        for j in 1..=upbound{
            if v[j] > 0 && v[l - j] > 0 {
                tmp += (v[j] * v[l - j]) % MOD;
            }
        }

        tmp = (tmp * C2(v[i])) % MOD;
        cnt = (cnt + tmp) % MOD;
    }

    println!("{}", cnt);
}

fn C2(n: u64) -> u64 {
    (n * (n - 1)) / 2
}
