fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut cnt = 0;
    let mut v = Vec::with_capacity(n);
    while v.len() < n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let tmp: Vec<usize> = buf
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        unsafe {
            let pointer: *mut usize = v.as_mut_ptr();
            std::ptr::copy_nonoverlapping(tmp.as_ptr(), pointer.add(v.len()), tmp.len());
            v.set_len(v.len() + tmp.len());
        }
    }

    // let mut cnt = 0;
    let mut record = vec![vec![0; m + 1]; n + 1];
    for i in 0..(n+1) {
        record[i][m] = 1;
    }
    let cnt = dp(&v, 0, 0, &m, &mut record);

    println!("{}", cnt);
}

fn dp(v: &Vec<usize>, i: usize, m: usize, total: &usize, record: &mut Vec<Vec<usize>>) -> usize {
    if i >= record.len() || *total < m {
        return 0;
    }
    if record[i][m] != 0 {
        return record[i][m];
    }

    let mut res = 0;
    for tmpi in i..v.len() {
        if m + v[tmpi] <= *total  {
            res += dp(v, tmpi + 1, m + v[tmpi], total, record);
        }
    }
    record[i][m] = res;
    res
}
