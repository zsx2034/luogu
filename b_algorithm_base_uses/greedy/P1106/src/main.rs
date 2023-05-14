use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut big_int: VecDeque<char> = buf.trim().chars().collect();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    if n >= big_int.len() || (big_int.len() == 1 && big_int[0] == '0') {
        println!("0");
        return;
    }

    for _ in 0..n {
        let mut flag = true;
        for i in 0..big_int.len() - 1 {
            if big_int[i] > big_int[i + 1] {
                big_int.remove(i);
                flag = false;
                break;
            }
        }

        if flag {
            big_int.pop_back();
        }

        while big_int.len() > 1 && big_int[0] == '0' {
            big_int.pop_front();
        }
    }

    let mut ans = String::new();
    for i in 0..big_int.len() {
        if big_int[i] != ' ' {
            ans.push(big_int[i]);
        }
    }
    println!("{}", ans);
}
