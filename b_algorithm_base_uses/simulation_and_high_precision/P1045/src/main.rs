use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: u32 = buf.trim().parse().unwrap();

    let tmp = n as f64;

    println!("{}", (2_f64.ln() / 10_f64.ln() * tmp + 1_f64) as i64);

    let mut record = HashMap::new();
    record.insert(1, vec![2]);

    let res = pow(n, &mut record);
    let mut res = res.clone();
    sub_one(&mut res);
    for i in (0..res.len()).rev() {
        print!("{:05}", res[i]);
        if i % 10 == 0 {
            println!();
        }
    }
}

fn sub_one(a: &mut Vec<u64>) {
    let mut sub = 1;
    for i in 0..a.len() {
        if a[i] >= sub {
            a[i] -= sub;
            break;
        } else {
            a[i] = 10_0000 + a[i] - sub;
            sub = 1;
        }
    }
}

fn pow(n: u32, record: &mut HashMap<u32, Vec<u64>>) -> &Vec<u64> {
    if record.get(&n).is_some() {
        return record.get(&n).unwrap();
    }

    if n % 2 == 0 {
        let tmp = pow(n / 2, record);
        let res = mul_mod(tmp, tmp);
        record.insert(n, res);
    } else {
        let tmp = pow(n / 2, record);
        let res = mul_mod(tmp, tmp);
        let res = mul_mod(&res, &vec![2]);
        record.insert(n, res);
    }

    return record.get(&n).unwrap();
}

// a.len() == 500
fn mul_mod(a: &Vec<u64>, b: &Vec<u64>) -> Vec<u64> {
    let mut res = vec![0; 100];
    for i in 0..a.len() {
        if i >= 100 {
            break;
        }
        for j in 0..b.len() {
            if j >= 100 {
                break;
            }

            if i + j >= 100 {
                continue;
            }

            res[i + j] += a[i] * b[j];
        }
    }

    for i in 0..res.len() - 1 {
        if res[i] >= 10_0000 {
            res[i + 1] += res[i] / 10_0000;
            res[i] %= 10_0000;
        }
    }

    let tmp = res.len() - 1;
    res[tmp] %= 10_0000;

    res
}

#[test]
fn test() {
    println!("{}", u64::max_value());
}