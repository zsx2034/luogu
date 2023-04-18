fn main() {
    let mut tmp = vec![false; 2000001];

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut input = buf.split_whitespace();
        let a = input.next().unwrap().trim().parse::<f64>().unwrap();
        let t = input.next().unwrap().trim().parse::<f64>().unwrap() + 1.0;

        let mut i = 1.0;
        while i <= t {
            let j = (a * i) as usize;
            tmp[j] = !tmp[j];
            i += 1.0;
        }
    }

    for (k, v) in tmp.iter().enumerate() {
        if *v {
            print!("{}", k);
        }
    }
}