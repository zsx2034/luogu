fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    let mut res = Vec::new();
    let mut rid = 0;
    for id in 1..n+1 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let num: Vec<char> = buf.trim().chars().collect();
        let tmp = max(res, num, rid, id);
        res = tmp.0;
        rid = tmp.1;
    }
    println!("{}", rid);

    for ch in res {
        print!("{}", ch);
    }
}

fn max<'a>(a: Vec<char>, b: Vec<char>, aid: usize, bid: usize) -> (Vec<char>, usize) {
    if a.len() == b.len() {
        for i in 0..a.len() {
            if a[i] > b[i] {
                return (a, aid);
            } else if a[i] < b[i] {
                return (b, bid);
            }
        }
        return (a, aid);
    } else if a.len() <= b.len() {
        return (b, bid);
    } else {
        return (a, aid);
    }
}
