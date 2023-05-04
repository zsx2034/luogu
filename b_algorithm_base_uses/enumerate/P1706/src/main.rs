fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();
    dp(&n, 0, &mut vec![0; n], &mut vec![false; n + 1]);
}

fn dp(n: &usize, dpeth: usize, memo: &mut Vec<usize>, is_read: &mut Vec<bool>) {
    if dpeth >= *n {
        for num in memo {
            print!("{:5}", num);
        }
        println!();
        return;
    }

    for i in 1..n + 1 {
        if is_read[i] {
            continue;
        }
        is_read[i] = true;
        memo[dpeth] = i;
        dp(n, dpeth + 1, memo, is_read);
        is_read[i] = false;
    }
}
