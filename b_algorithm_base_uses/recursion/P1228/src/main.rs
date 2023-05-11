fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let col: usize = iter.next().unwrap().parse().unwrap();
    let row: usize = iter.next().unwrap().parse().unwrap();

    let l = 2_usize.pow(n as u32);

    solve(col, row, 1, 1, l);
}

fn solve(col: usize, row: usize, a: usize, b: usize, l: usize) {
    if l == 1 {
        return;
    }

    if col < l / 2 + a && row < l / 2 + b {
        println!("{} {} 1", a + l / 2, b + l / 2);
        solve(col, row, a, b, l / 2);
        solve(a + l / 2 - 1, b + l / 2, a, b + l / 2, l / 2);
        solve(a + l / 2, b + l / 2 - 1, a + l / 2, b, l / 2);
        solve(a + l / 2, b + l / 2, a + l / 2, b + l / 2, l / 2);
    } else if (col < l / 2 + a && row + 1 > l / 2 + b) {
        println!("{} {} 2", a + l / 2, b + l / 2 - 1);
        solve(a + l / 2 - 1, b + l / 2 - 1, a, b, l / 2);
        solve(col, row, a, b + l / 2, l / 2);
        solve(a + l / 2, b + l / 2 - 1, a + l / 2, b, l / 2);
        solve(a + l / 2, b + l / 2, a + l / 2, b + l / 2, l / 2);
    } else if (col + 1 > l / 2 + a && row + 1 <= l / 2 + b) {
        println!("{} {} 3", a + l / 2 - 1, b + l / 2);
        solve(a + l / 2 - 1, b + l / 2 - 1, a, b, l / 2);
        solve(a + l / 2 - 1, b + l / 2, a, b + l / 2, l / 2);
        solve(col, row, a + l / 2, b, l / 2);
        solve(a + l / 2, b + l / 2, a + l / 2, b + l / 2, l / 2);
    } else {
        println!("{} {} 4", a + l / 2 - 1, b + l / 2 - 1);
        solve(a + l / 2 - 1, b + l / 2 - 1, a, b, l / 2);
        solve(a + l / 2 - 1, b + l / 2, a, b + l / 2, l / 2);
        solve(a + l / 2, b + l / 2 - 1, a + l / 2, b, l / 2);
        solve(col, row, a + l / 2, b + l / 2, l / 2);
    }
}
