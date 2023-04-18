fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let name = iter.next().unwrap();
        let age: i32 = iter.next().unwrap().parse().unwrap();
        let score: i32 = iter.next().unwrap().parse().unwrap();
        println!("{} {} {}", name, age + 1 , {
            let tmp = score * 12;
            if tmp > 6000 {
                600
            } else {
                tmp / 10
            }
        });
    }
}
