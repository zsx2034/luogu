fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: u64 = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut v: Vec<u64> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    v.sort();

    let mut l = 0;
    let mut r = v[n - 1];
    let mut s = 0;
    let mut mid = 0;
    while l <= r {
        mid = (l + r) / 2;
        s = sum(&v, mid);
        match s.cmp(&m) {
            std::cmp::Ordering::Less => {
                if mid == 0 {
                    break;
                }
                r = mid - 1;
            },
            std::cmp::Ordering::Equal => {
                println!("{}", mid);
                return;
            },
            std::cmp::Ordering::Greater => {
                l = mid + 1;
            },
        }
    } 

    if s < m {
        println!("{}", mid - 1);
    } else {
        println!("{}", mid);
    }

}

fn sum(v: &Vec<u64>, h: u64) -> u64 {
    let mut sum = 0;
    for i in v.iter().rev() {
        if *i <= h {
            break;
        }
        sum += i - h;
    }
    sum
}
