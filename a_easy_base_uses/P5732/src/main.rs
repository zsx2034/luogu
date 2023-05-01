use std::mem::swap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();

    let mut last = vec![1; n];
    let mut cur = vec![1; n];
    for i in 0..n {
        for j in 0..i + 1 {
            if j == 0 || j == i {
                print!("{} ", 1);
                cur[j] = 1;
            } else {
                print!("{} ", last[j] + last[j - 1]);
                cur[j] = last[j] + last[j - 1];
            }
        }
        println!();
        swap(&mut last, &mut cur);
    }
}
