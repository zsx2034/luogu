fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let n: u32 = iter.next().unwrap().parse().unwrap();
    let r: u32 = iter.next().unwrap().parse().unwrap();

    dp(n, r, 1, &mut vec![0; r as usize], 0);
}

fn dp(n: u32, r: u32, i: u32, p: &mut Vec<u32>, cnt: u32) {
    if cnt == r {
        for i in 0..r {
            print!("{:3}", p[i as usize]);
        }
        println!();
        return;
    }

    for j in i..(n + 1) {
        p[cnt as usize] = j;
        dp(n, r, j + 1, p, cnt + 1);
    }
}
