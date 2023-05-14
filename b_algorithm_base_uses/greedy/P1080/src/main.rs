const STEP: u128 = 100_0000_0000;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let a: u128 = iter.next().unwrap().parse().unwrap();
    let b: u128 = iter.next().unwrap().parse().unwrap();

    let mut v: Vec<(u128, u128)> = Vec::new();
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let x: u128 = iter.next().unwrap().parse().unwrap();
        let y: u128 = iter.next().unwrap().parse().unwrap();
        v.push((x, y));
    }

    v.sort_by(|a, b| {
        let a = a.0 * a.1;
        let b = b.0 * b.1;
        a.partial_cmp(&b).unwrap()
    });

    let mut ans = vec![0];
    let mut tmp = num2vec(a);
    for i in 0..n {
        let left = v[i].0;
        let right = v[i].1;
        let _tmp = div(&tmp, right);
        ans = max(ans, _tmp);
        mul(&mut tmp, left);
    }

    print!("{}", ans[ans.len() - 1]);
    for i in (0..ans.len() - 1).rev() {
        print!("{:0>10}", ans[i]);
    }
}

fn mul(v: &mut Vec<u128>, n: u128) {
    for i in 0..v.len() {
        v[i] *= n;
    }

    for i in 0..v.len() - 1 {
        if v[i] >= STEP {
            v[i + 1] += v[i] / STEP;
            v[i] %= STEP;
        }
    }

    while v[v.len() - 1] >= STEP {
        let l = v.len() - 1;
        let tmp = v[l] / STEP;
        v[l] %= STEP;
        v.push(tmp);
    }
}

fn div(v: &Vec<u128>, n: u128) -> Vec<u128> {
    let mut v = v.clone();
    let mut tmp = 0;
    for i in (0..v.len()).rev() {
        tmp *= STEP;
        tmp += v[i];
        v[i] = tmp / n;
        tmp %= n;
    }

    while v.len() > 1 && v[v.len() - 1] == 0 {
        v.pop();
    }

    v
}

fn max(a: Vec<u128>, b: Vec<u128>) -> Vec<u128> {
    match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Less => b,
        std::cmp::Ordering::Greater => a,
        std::cmp::Ordering::Equal => {
            for i in (0..a.len()).rev() {
                match a[i].cmp(&b[i]) {
                    std::cmp::Ordering::Less => return b,
                    std::cmp::Ordering::Greater => return a,
                    std::cmp::Ordering::Equal => {}
                }
            }
            a
        }
    }
}

fn num2vec(mut n: u128) -> Vec<u128> {
    let mut v = Vec::new();
    while n > 0 {
        v.push(n % STEP);
        n /= STEP;
    }
    v
}
