use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut input = buf
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap());
    let n = input.next().unwrap();
    let m = input.next().unwrap();

    let mut sum_tmp = 0;
    let mut record_tmp = Vec::new();

    for _ in 0..m {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let ai = buf.trim().parse::<u32>().unwrap();
        record_tmp.push(ai);
        sum_tmp += ai;
    }

    let mut min_tmp = sum_tmp;
    for i in m..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let ai = buf.trim().parse::<u32>().unwrap();

        sum_tmp = sum_tmp + ai - record_tmp[(i-m)];
 
        min_tmp = min(min_tmp, sum_tmp);
 
        record_tmp.push(ai);
    }


    println!("{}", min_tmp);
}
