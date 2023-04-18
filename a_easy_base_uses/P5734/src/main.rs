fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut str = buf.trim().to_string();

    for _ in 0..n {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut input = buf.trim().split_whitespace();
        let op: u8 = input.next().unwrap().parse().unwrap();
        match op {
            1 => {
                let args = input.next().unwrap();
                str.push_str(args);
                println!("{}", str);
            },
            2 => {
                let args_a: usize = input.next().unwrap().parse().unwrap();
                let args_b: usize = input.next().unwrap().parse().unwrap();
                
                str = str.split_off(args_a);
                str.truncate(args_b);
                println!("{}", str);
            },
            3 => {
                let args_a: usize = input.next().unwrap().parse().unwrap();
                let args_b: &str = input.next().unwrap();

                str.insert_str(args_a, args_b);
                println!("{}", str);
            },
            4 => {
                let args_a: &str = input.next().unwrap();
                match str.find(args_a) {
                    Some(i) => println!("{}", i),
                    None => println!("-1"),
                }
            }
            _ => {}
        }
    }
}
