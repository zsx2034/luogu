fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let mut row: u64 = iter.next().unwrap().parse::<u64>().unwrap() + 1;
    let mut col: u64 = iter.next().unwrap().parse::<u64>().unwrap() + 1;

    let total = row * (row - 1) * col * (col - 1) / 4;

    let mut cnt = 0;
    let col_tmp = col - 1;
    for i in 1..row {
        let row_tmp = row - i;
        if row_tmp >= col_tmp {
            cnt += col_tmp * (col_tmp + 1) / 2;
        } else {
            cnt += row_tmp * col_tmp - row_tmp * (row_tmp - 1) / 2;
        }
    }


    println!("{} {}", cnt, total - cnt);
}
