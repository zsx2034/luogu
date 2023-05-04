fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v = buf
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut cnt = 0;
    dp(&v, k, 0, 0, 0, &mut cnt);
    println!("{}", cnt);
}

fn dp(v: &Vec<i32>, k: i32, i: i32, j: usize, sum: i32, cnt: &mut i32) {
    if i == k {
        if is_prime(sum) {
            *cnt += 1;
        }
        return;
    }

    if j >= v.len() {
        return ;
    }

    for index in j..v.len() {
        dp(v, k, i + 1, index + 1, sum + v[index], cnt);
    }
}

fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    let upbound = (num as f64).sqrt() as i32;
    for i in 2..upbound + 1 {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}
