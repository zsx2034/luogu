fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let row: usize = iter.next().unwrap().parse().unwrap();
    let col: usize = iter.next().unwrap().parse().unwrap();

    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut map: Vec<Vec<char>> = Vec::with_capacity(row);
    for i in 0..row {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        map.push(buf.trim().chars().collect());
    }

    let mut res = 0;

    // row
    for i in 0..row {
        let mut cnt = 0;
        for j in 0..col {
            if map[i][j] == '.' {
                cnt += 1;
            } else {
                if cnt >= k {
                    res += cnt - k + 1;
                }
                cnt = 0;
            }
        }
        if cnt >= k {
            res += cnt - k + 1;
        }
    }

    // col
    for j in 0..col {
        let mut cnt = 0;
        for i in 0..row {
            if map[i][j] == '.' {
                cnt += 1;
            } else {
                if cnt >= k {
                    res += cnt - k + 1;
                }
                cnt = 0;
            }
        }
        if cnt >= k {
            res += cnt - k + 1;
        }
    }

    if k == 1 {
        res /= 2;
    }
    println!("{}", res);

}
