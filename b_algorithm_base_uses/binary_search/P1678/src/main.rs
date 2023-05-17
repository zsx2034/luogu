fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut schools: Vec<usize> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    schools.sort();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let students: Vec<usize> = buf
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut ans = 0;
    for s in students {
        let i = lower_bound(&schools, &s);
        if i == 0 {
            ans += schools[i] - s;
        } else if i == n {
            ans += s - schools[i - 1];
        } else {
            ans += std::cmp::min(s - schools[i - 1], schools[i] - s);
        }
    }

    println!("{}", ans);
}

// find the one that is greater than or equal to the given value
fn lower_bound(v: &Vec<usize>, t: &usize) -> usize {
    let mut l = 0;
    let mut r = v.len();
    while l < r {
        let m = (l + r) / 2;
        if v[m] < *t {
            l = m + 1;
        } else {
            r = m;
        }
    }
    l
}