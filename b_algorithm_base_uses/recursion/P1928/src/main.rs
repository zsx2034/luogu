fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v = buf.trim().chars().collect::<Vec<char>>();
    let (_, res) = decompression(&v);

    println!("{}", res);
}

fn decompression(v: &[char]) -> (usize, String) {
    let mut res = String::new();

    let mut i = 0;
    let mut cnt = 1;

    while i < v.len() {
        let ch = v[i].to_string();
        if v[i].is_ascii_digit() {
            let mut tmpi = i;
            while tmpi < v.len() && v[tmpi].is_ascii_digit() {
                tmpi += 1;
            }
            cnt = v[i..tmpi].iter().collect::<String>().parse::<usize>().unwrap();
            i = tmpi;
        } else if v[i].is_alphabetic() {
            res.push(v[i]);
            i += 1;
        } else if v[i] == '[' {
            let (tmpi, tmps) = decompression(&v[i + 1..]);
            res.push_str(&tmps);
            i += tmpi+2;
        } else if v[i] == ']' {
            return (i, res.repeat(cnt));
        }
    }
    (v.len(), res.repeat(cnt))
}
