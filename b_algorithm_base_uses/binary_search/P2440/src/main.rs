fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: u32 = iter.next().unwrap().parse().unwrap();

    let mut v: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        v.push(buf.trim().parse().unwrap());
    }

    v.sort();

    let mut l = 0;
    let mut r = 1_0000_0001;
    let mut mid = 0;
    while l < r {
        mid = (l + r) / 2;
        let tmp = count(&v, mid);
        if tmp < k {
            r = mid - 1;
        } else {
            if l == mid {
                break;
            }
            l = mid;
        }
    }

    if count(&v, l + 1) >= k {
        println!("{}", l + 1);
    } else {
        println!("{}", l);
    }

}

fn count(v: &Vec<u32>, l: u32) -> u32{
    let mut count = 0;
    for i in v.iter().rev() {
        let tmp = i / l;
        if tmp == 0 {
            break;
        }
        count += tmp;
    }
    count
}
