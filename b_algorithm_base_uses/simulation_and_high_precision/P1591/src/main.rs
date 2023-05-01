fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    let mut record = Vec::with_capacity(1000);
    record.push(vec![1]);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: u16 = iter.next().unwrap().parse().unwrap();
        if record.get(a).is_none() {
            calc(&mut record, a);
        }

        let mut cnt = 0;
        for i in record[a].iter() {
            if *i == b {
                cnt += 1;
            }
        }

        if b == 0 {
            for i in record[a].iter().rev() {
                if *i == 0 {
                    cnt -= 1;
                } else {
                    break;
                }
            }
        }
        println!("{}", cnt);
    }
}

fn calc(record: &mut Vec<Vec<u16>>, n: usize) {
    if record.get(n).is_some() {
        return;
    }
    if n == 1 {
        record.push(vec![1]);
        return;
    }

    if record.get(n - 1).is_none() {
        calc(record, n - 1);
    }

    // transfer to u16
    let mut b = Vec::with_capacity(5);
    let mut temp = n;
    while temp > 0 {
        b.push((temp % 10) as u16);
        temp /= 10;
    }

    record.push(mul(&record[n - 1], &b));
}


fn mul(a: &Vec<u16>, b: &Vec<u16>) -> Vec<u16> {
    let mut result = vec![0; a.len() + b.len()];
    for i in 0..a.len() {
        for j in 0..b.len() {
            let mut temp = a[i] * b[j];
            result[i + j] += temp;
        }
    }

    for i in 0..result.len() - 1 {
        result[i + 1] += result[i] / 10;
        result[i] %= 10;
    }

    result
}
