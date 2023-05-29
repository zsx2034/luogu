fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let num: i64 = buf.trim().parse().unwrap();

    let mut record = vec![0; num as usize];
    solve(num, 0, 0, 1,&mut record);
}

fn solve(num: i64, depth: i64, sum: i64, last: i64, record: &mut Vec<i64>) {
    if sum == num && depth == 1 {
        return ;
    }
    if sum == num {
        let depth = depth as usize - 1;
        for (i, num) in record.iter().enumerate() {
            print!("{}", num);
            if i != depth {
                print!("+");
            } else {
                break;
            }
        }
        println!();
        return;
    }

    for i in last..=num - sum {
        record[depth as usize] = i;
        solve(num, depth + 1, sum + i, i,record);
    }
}
