fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut short = buf
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut long = buf
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    if short.len() > long.len() {
        std::mem::swap(&mut short, &mut long);
    }

    if short.len() == 1 && short[0] == 0 || long.len() == 1 && long[0] == 0 {
        println!("0");
        return 
    }

    let mut res = vec![0;long.len()+ short.len()];

    for i in (0..short.len()).rev() {
        for j in (0..long.len()).rev() {
            res[i+j+1] += short[i] * long[j];
        }
    }

    for i in (0..res.len()).rev() {
        if res[i] > 9 {
            res[i - 1] += res[i] / 10;
            res[i] %= 10;
        }
    }

    let mut flag = true;
    for i in 0..res.len() {
        if flag && res[i] == 0 {
            continue;
        } else {
            flag = false;
            print!("{}", res[i]);
        }
    }
}
