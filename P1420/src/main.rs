use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n = buf.trim().parse::<i128>().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let mut data: VecDeque<_> = buf
        .split_whitespace()
        .map(|num| num.parse::<i128>().unwrap())
        .collect();

    if data.is_empty() {
        println!("{}", 0);
    }

    let mut pre = data.pop_front().unwrap();
    let mut cnt = 1;
    let mut max_cnt = 1;
    for num in &data {
        if num == &(pre + 1) {
            cnt += 1;
            pre = *num;
        } else {
            if cnt > max_cnt {
                max_cnt = cnt;
            }
            cnt = 1;
            pre = *num;
        }
    }

    println!("{}", if cnt > max_cnt { cnt } else { max_cnt });
}
