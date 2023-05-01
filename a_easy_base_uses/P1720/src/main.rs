fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("msg");
    let n = buf.trim().parse::<i128>().unwrap();

    match n {
        0 => {
            println!("{}.00", 0);
        }
        1 => {
            println!("{}.00", 1);
        }
        2 => {
            println!("{}.00", 1);
        }
        _ => {
            let mut a: i128 = 1;
            let mut b: i128 = 1;
            let mut c: i128 = a + b;
            for i in 2..n {
                a = b;
                b = c;
                c = a + b;
            }
            println!("{}.00", b);
        }
    };
}

