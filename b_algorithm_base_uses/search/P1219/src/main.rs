fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut v = vec![0; n];

    let mut cnt = 0;
    let mut row = vec![false; n];
    let mut diagonal = vec![false; 2 * n - 1];
    let mut ndiaonal = vec![false; 2 * n - 1];
    dp(&mut v, &mut row, &mut diagonal,&mut ndiaonal,0, &mut cnt);
    println!("{}", cnt);
}

fn dp(v: &mut Vec<usize>,row: &mut Vec<bool>, diagonal: &mut Vec<bool>, ndiagonal: &mut Vec<bool>, dep: usize, cnt: &mut usize) {
    if dep == v.len() {
        if *cnt < 3 {
            for i in 0..v.len() {
                print!("{} ", v[i] + 1);
            }
            println!();
        }
        *cnt += 1;
        return;
    }

    for i in 0..v.len() {
        let tmp = i +  v.len() - 1 - dep;
        let ntmp = i + dep;
        if row[i] || diagonal[tmp] || ndiagonal[ntmp] {
            continue;
        }
        row[i] = true;
        diagonal[tmp] = true;
        ndiagonal[ntmp] = true;
        v[dep] = i;
        dp(v, row, diagonal, ndiagonal, dep + 1, cnt);
        row[i] = false;
        diagonal[tmp] = false;
        ndiagonal[ntmp] = false;
    }
}
