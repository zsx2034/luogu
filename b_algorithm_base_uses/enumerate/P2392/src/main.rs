use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let s1: usize = iter.next().unwrap().parse().unwrap();
    let s2: usize = iter.next().unwrap().parse().unwrap();
    let s3: usize = iter.next().unwrap().parse().unwrap();
    let s4: usize = iter.next().unwrap().parse().unwrap();

    let mut sum = 0;

    for _ in 0..4 {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let v = buf.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let mut min = std::u32::MAX;
        dp(&v, 0, 0, 0, &mut min);
        sum += min;
    }

    println!("{}", sum);
}

fn dp(v: &Vec<u32>, i: usize, sum1: u32, sum2: u32, min: &mut u32) {
    if i == v.len() {
        if *min > max(sum1, sum2) {
            *min = max(sum1, sum2);
        }
        return;
    }

    dp(v, i + 1, sum1 + v[i], sum2, min);
    dp(v, i + 1, sum1, sum2 + v[i], min);
}