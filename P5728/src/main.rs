fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n: usize = buf.trim().parse().unwrap();
    let mut stus = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).expect("msg");
        let mut data: Vec<i32> = buf
            .split_whitespace()
            .map(|num| num.trim().parse().unwrap())
            .collect();
        data.push(data.iter().sum());
        stus.push(data);
    }

    let mut cnt = 0;
    for i in 0..n {
        for j in i + 1..n {
            if same(&stus[i], &stus[j]) {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}

fn same(a: &[i32], b: &[i32]) -> bool {
    for i in 0..3 {
        if (a[i] - b[i]).abs() > 5 {
            return false;
        }
    }
    if (a[3] - b[3]).abs() > 10 {
        return false;
    }
    return true;
}
