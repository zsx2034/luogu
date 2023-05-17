fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let _length: u32  = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut v: Vec<u32> = Vec::with_capacity(n + 1);
    v.push(0);
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        v.push(buf.trim().parse().unwrap());
    }
    v.push(_length);

    let mut length: Vec<u32> = vec![0;n+1];
    for i in 0..n+1 {
        length[i] = v[i+1] - v[i];
    }

    let mut l = 0;
    let mut r = _length + 1;

    while l < r {
        let mid = (l + r) / 2;
        if check(&length, mid, k) {
            if l == mid {
                if check(&length, l+1, k) {
                    l += 1;
                }
                break;
            }
            l = mid;
        } else {
            r = mid - 1;
        }
    }

    println!("{}", l);
}

fn check(v: &Vec<u32>, l: u32, mut k: usize) -> bool {
    let mut tmp = 0;
    let mut i = 0;
    let mut j = 0;
    while i < v.len() {
        if v[i] >= l {
            i += 1;
        } else {
            j = i;
            j += 1;
            tmp = v[i];
            while j < v.len() && tmp < l {
                tmp += v[j];
                if k > 0 {
                    k -= 1;
                } else {
                    return false;
                }
                j += 1;
            }

            if tmp < l {
                if i > 0 && k > 0 {
                    k -= 1;
                } else {
                    return false;
                }
            }
            i = j;
        }
    }
    true
}
