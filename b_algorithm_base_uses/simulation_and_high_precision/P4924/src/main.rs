fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut num = 1;
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(Vec::with_capacity(n));
        for _ in 0..n {
            v[i].push(num);
            num += 1;
        }
    }

    let mut tmp = vec![vec![0; n]; n];

    for _ in 0..m {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_ascii_whitespace();
        let y: usize = iter.next().unwrap().parse().unwrap();
        let y = y - 1;
        let x: usize = iter.next().unwrap().parse().unwrap();
        let x = x - 1;
        let r: usize = iter.next().unwrap().parse().unwrap();
        let op: usize = iter.next().unwrap().parse().unwrap();

        if op == 0 {
            // 将 (x - r, y - r) 为左上角，(x + r, y + r) 为右下角的矩形区域内的所有数字，顺时针旋转 90 度。
            let mut new_line = y - r;
            let mut new_col = x - r;
            
            for ori_col in x - r..x + r + 1 {
                new_col = x - r;
                for ori_line in (y - r..y + r + 1).rev() {
                    tmp[new_line][new_col] = v[ori_line][ori_col];
                    new_col += 1;
                }
                new_line += 1;
            }
        } else {
            // 将 (x - r, y - r) 为左上角，(x + r, y + r) 为右下角的矩形区域内的所有数字，逆时针旋转 90 度。
            let mut new_line = y - r;
            let mut new_col = x - r;
            for ori_col in (x - r..x + r + 1).rev() {
                new_col = x - r;
                for ori_line in y - r..y + r + 1 {
                    tmp[new_line][new_col] = v[ori_line][ori_col];
                    new_col += 1;
                }
                new_line += 1;
            }
        }

        for i in y - r..y + r + 1 {
            for j in x - r..x + r + 1 {
                v[i][j] = tmp[i][j];
            }
        }
    }


    for i in 0..n {
        for j in 0..n {
            print!("{} ", v[i][j]);
        }
        println!();
    }
}
