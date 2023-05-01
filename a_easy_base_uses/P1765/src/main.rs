fn main() {
    let num: [i32; 26] = [
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 4, 1, 2, 3, 1, 2, 3, 4,
    ];

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();buf.pop(); // remove \r\n
    let chars = buf.chars().collect::<Vec<_>>();

    let mut res_cnt = 0;
    for ch in chars {
        if ch == ' ' {
            res_cnt += 1;
        } else {
            res_cnt += num[(ch as u8 - b'a') as usize];
        }
    }

    println!("{}", res_cnt);
}
