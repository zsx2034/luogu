fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    if n < 10 || n > 30 {
        println!("0");
        return;
    }

    let mut res = Vec::new();
    let mut tmp = vec![0; 10];
    peiliao(0, n, &mut res, &mut tmp);

    println!("{}", res.len());

    for i in 0..res.len() {
        for j in 0..9 {
            print!("{} ", res[i][j]);
        }
        println!("{}", res[i][9]);
    }
}

fn peiliao(kinds: usize, n: usize, record: &mut Vec<Vec<usize>>, tmp: &mut Vec<usize>) {
    if kinds == 10 {
        if n == 0 {
            record.push(tmp.clone());
        }
        return;
    }

    for i in 1..4 {
        if n < i {
            return;
        }
        tmp[kinds] = i;
        peiliao(kinds + 1, n - i, record, tmp);
    }
}
