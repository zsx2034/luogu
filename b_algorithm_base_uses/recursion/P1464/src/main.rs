fn main() {
    let mut buf = String::new();
    let mut running = true;
    let mut v = vec![vec![vec![0; 21]; 21]; 21];
    while running {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();

        let mut iter = buf.split_whitespace();
        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();

        if a == -1 && b == -1 && c == -1 {
            running = false;
            continue;
        }

        println!("w({}, {}, {}) = {}", a, b, c, solve(&mut v, a, b, c));

        // for h in v.iter() {
        //     for i in h.iter() {
        //         println!("{:?}", i);
        //     }
        //     println!();
        // }
    }
}

fn solve(v: &mut Vec<Vec<Vec<i64>>>, a: i64, b: i64, c: i64) -> i64 {
    if a <= 0 || b <= 0 || c <= 0 {
        return 1;
    }

    if a > 20 || b > 20 || c > 20 {
        return solve(v, 20, 20, 20);
    }

    if v[a as usize][b as usize][c as usize] != 0 {
        return v[a as usize][b as usize][c as usize];
    }

    if a < b && b < c {
        v[a as usize][b as usize][c as usize] =
            solve(v, a, b, c - 1) + solve(v, a, b - 1, c - 1) - solve(v, a, b - 1, c);
        // println!(
        //     "a: {} b: {} c: {}, v: {}",
        //     a, b, c, v[a as usize][b as usize][c as usize]
        // );
        return v[a as usize][b as usize][c as usize];
    }

    v[a as usize][b as usize][c as usize] =
        solve(v, a - 1, b, c) + solve(v, a - 1, b - 1, c) + solve(v, a - 1, b, c - 1)
            - solve(v, a - 1, b - 1, c - 1);
    // println!(
    //     "a: {} b: {} c: {}, v: {}",
    //     a, b, c, v[a as usize][b as usize][c as usize]
    // );

    return v[a as usize][b as usize][c as usize];
}
