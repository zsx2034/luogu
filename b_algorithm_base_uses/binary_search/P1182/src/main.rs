fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<u32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut l = 0;
    let mut r = 10_0000_0001;

    while r > l {
        let mid = (l + r) / 2;
        if check(&v, m, mid) {
            r = mid;
            if l == mid {
                break;
            }
        } else {
            l = mid + 1;
        }
    }

    println!("{}", r);

}

fn check(v: &Vec<u32>, m: usize, x: u32) -> bool {
    let mut count = 0;
    let mut sum = 0;
    let mut i = 0;
    while i < v.len() {
        if v[i] <= x {
            let mut j = i;
            j += 1;
            sum = v[i];
            while j < v.len() && sum + v[j] <= x {
                sum += v[j];
                j += 1;
            }
            i = j;
        } else {
            return false;
        }

        count += 1;
    }
    count <= m
}
