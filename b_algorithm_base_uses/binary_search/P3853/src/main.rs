fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let mut len: u32 = iter.next().unwrap().parse().unwrap();
    let mut n: usize = iter.next().unwrap().parse().unwrap();
    let mut k: usize = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<u32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut length = vec![0; v.len() - 1];

    for i in 1..v.len() {
        length[i - 1] = v[i] - v[i - 1];
    }

    let mut l = 0;
    let mut r = len;
    while l < r {
        let mid = (l + r) / 2;
        if check(&length, mid, k) {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    println!("{}", l);
}

fn check(v: &Vec<u32>, len: u32, mut k: usize) -> bool {
    let mut i = 0;
    while i < v.len() {
        if v[i] > len {
            let mut tmp = v[i];
            let mut cnt = 1;
            while tmp > len {
                if k == 0 {
                    return false;
                }
                k -= 1;
                cnt += 1;
                tmp = if v[i] % cnt == 0 {
                    v[i] / cnt
                } else {
                    v[i] / cnt + 1
                };
            }
        }
        i += 1;
    }
    true
}
