fn main() {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("error while input");
    let full_size: Vec<usize> = buf
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    let mut total: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![true; full_size[2]]; full_size[1]]; full_size[0]];
    buf.clear();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("error while input");
    let n: usize = buf.trim().parse().unwrap();
    for _ in 0..n {
        buf.clear();
        std::io::stdin()
            .read_line(&mut buf)
            .expect("error while input");
        let range: Vec<usize> = buf
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        for layer in total.iter_mut().take(range[3]).skip(range[0]-1) {
            for row in layer.iter_mut().take(range[4]).skip(range[1]-1) {
                for item in row.iter_mut().take(range[5]).skip(range[2]-1) {
                    *item = false;
                }
            }
        }
    }

    let mut cnt = 0;
    for layer in total.iter() {
        for row in layer.iter() {
            for item in row.iter() {
                if *item {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
