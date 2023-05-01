use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut nums: VecDeque<usize> = buf.split_whitespace()
        .map(|s| s.trim().parse().unwrap()).collect();
    let len_map = nums.pop_front().unwrap();

    let mut it = nums.iter();
    let mut x = *it.next().unwrap();
    let mut cnt = 0;
    let mut sign = 0;
    for i in 0..len_map {
        for j in 0..len_map {
            if cnt >= x {
                sign = if sign == 0 { 1 } else { 0 };
                x = *it.next().unwrap();
                cnt = 0;
            } 
            print!("{}", sign);
            cnt += 1;
        }

        println!();
    } 
}


